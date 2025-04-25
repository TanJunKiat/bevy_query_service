// =========================================================================
/*
 * Copyright (C) 2019 Tan Jun Kiat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// =========================================================================
use super::*;

/// A system that listens to query requests
/// `T` is the query request content
/// `U` is the query reply content
pub fn spawn_request_endpoint<T, U>(mut commands: Commands, mut events: EventReader<QueryEvent<T>>)
where
    T: Clone + Send + Sync + 'static,
    U: Default + Send + Sync + 'static,
{
    for event in events.read() {
        commands.spawn((GoalComponent::new(event.uuid), QueryRequest { request: event.request.clone() }, QueryReply::<U>::default()));
        info!("[{:?}]: Request spawned", event.uuid);
    }
}

/// `T` is the query request content
/// `U` is the query reply content
pub fn run_query_server<T, U>(world: &mut World)
where
    T: Send + Sync + 'static + Clone,
    U: QueryServerOps<T> + Send + Sync + 'static + Clone,
{
    let mut entities = Vec::new();
    let mut query_queries = world.query_filtered::<(Entity, &mut GoalComponent, &QueryRequest<T>), (With<GoalComponent>, With<QueryRequest<T>>)>();

    for (entity, mut goal, request) in query_queries.iter_mut(world) {
        if goal.is_executing() {
            debug!("[{:?}]: Goal is already executing", goal.get_uuid());
            continue;
        }

        if goal.is_completed() {
            debug!("[{:?}]: Goal is already completed", goal.get_uuid());
            continue;
        }

        if goal.is_to_delete() {
            debug!("[{:?}]: Goal is marked for deletion", goal.get_uuid());
            continue;
        }

        goal.mark_executing();

        entities.push((entity, goal.clone(), request.clone()));
    }

    for (entity, goal, request) in entities.iter_mut() {
        match U::get_reply(world, request) {
            Ok(reply) => {
                info!("[{:?}]: Goal is completed", goal.get_uuid());
                goal.mark_completed();
                world.get_entity_mut(*entity).unwrap().insert((goal.clone(), QueryReply { reply: reply }));
            }
            Err(_) => {
                error!("[{:?}]: Query reply failed", goal.get_uuid());
                goal.mark_to_delete();
            }
        }
    }
}

/// Garbage collection for query requests
/// todo: implement a timeout check
pub fn cleanup_requests(mut commands: Commands, queries: Query<(Entity, &GoalComponent), With<GoalComponent>>) {
    for (entity, goal) in queries.iter() {
        if goal.is_to_delete() {
            commands.entity(entity).despawn();
            info!("[{:?}]: Request despawned", goal.get_uuid());
        }
    }
}

use bevy_tokio_tasks::TokioTasksRuntime;
pub fn run_query_client<T, U>(runtime: ResMut<TokioTasksRuntime>, mut query_queries: Query<(Entity, &mut GoalComponent, &QueryRequest<T>), (With<GoalComponent>, With<QueryRequest<T>>)>)
where
    T: Send + Sync + 'static + Clone,
    U: QueryClientOps<T> + Send + Sync + 'static + Clone,
{
    let mut entities = Vec::new();
    for (entity, mut goal, request) in query_queries.iter_mut() {
        if goal.is_executing() {
            debug!("[{:?}]: Goal is already executing", goal.get_uuid());
            continue;
        }

        if goal.is_completed() {
            debug!("[{:?}]: Goal is already completed", goal.get_uuid());
            continue;
        }
        if goal.is_to_delete() {
            debug!("[{:?}]: Goal is marked for deletion", goal.get_uuid());
            continue;
        }

        goal.mark_executing();

        entities.push((entity, goal.clone(), request.clone()));
    }

    for (entity, goal, request) in entities.iter_mut() {
        let mut goal = goal.clone();
        let entity_clone = entity.clone();
        let request = request.clone();
        runtime.spawn_background_task(move |mut ctx| async move {
            match U::send_request(&mut ctx, &request).await {
                Ok(reply) => {
                    ctx.run_on_main_thread(move |ctx| {
                        info!("[{:?}]: Goal is completed", goal.get_uuid());
                        goal.mark_completed();
                        ctx.world.get_entity_mut(entity_clone).unwrap().insert((goal.clone(), QueryReply { reply: reply }));
                    })
                    .await;
                }
                Err(_) => {
                    ctx.run_on_main_thread(move |ctx| {
                        error!("[{:?}]: Query reply failed", goal.get_uuid());
                        goal.mark_to_delete();
                        ctx.world.get_entity_mut(entity_clone).unwrap().insert(goal.clone());
                    })
                    .await;
                }
            }
        });
    }
}
