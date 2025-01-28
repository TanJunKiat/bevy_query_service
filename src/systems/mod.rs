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
        commands.spawn((
            GoalComponent {
                uuid: event.uuid,
                ..Default::default()
            },
            QueryRequest { request: event.request.clone() },
            QueryReply::<U>::default(),
        ));
        info!("[{:?}]: Request spawned", event.uuid);
    }
}

/// `T` is the query request content
/// `U` is the query reply content
pub fn run_query_server<T, U>(mut query_queries: Query<(&mut GoalComponent, &QueryRequest<T>, &mut QueryReply<U>), (With<GoalComponent>, With<QueryRequest<T>>, With<QueryReply<U>>)>)
where
    T: Send + Sync + 'static,
    U: QueryReplyOpsSingle<T> + Send + Sync + 'static,
{
    for (mut goal, request, mut reply) in query_queries.iter_mut() {
        if goal.is_completed {
            debug!("[{:?}]: Goal is already completed", goal.uuid);
            continue;
        }

        reply.reply = U::get_reply(request);

        goal.is_completed = true;
        info!("[{:?}]: Goal is completed", goal.uuid);
    }
}

/// `T` is the query request content
/// `U` is the query reply content
/// `V` is the additional query that the server can use to calculate the reply
pub fn run_query_server_with_supplement<T, U, V>(
    mut query_queries: Query<(&mut GoalComponent, &QueryRequest<T>, &mut QueryReply<U>), (With<GoalComponent>, With<QueryRequest<T>>, With<QueryReply<U>>)>,
    supplementary_queries: Query<&V, With<V>>,
) where
    T: Send + Sync + 'static,
    U: QueryReplyOpsDouble<T, V> + Send + Sync + 'static,
    V: Component,
{
    for (mut goal, request, mut reply) in query_queries.iter_mut() {
        if goal.is_completed {
            debug!("[{:?}]: Goal is already completed", goal.uuid);
            continue;
        }

        match  U::get_reply(request, &supplementary_queries) {
            Ok(r) => reply.reply = r,
            Err(_) => {
                error!("[{:?}]: Failed to get reply", goal.uuid);
                continue;
            }
        }

        goal.is_completed = true;
        info!("[{:?}]: Goal is completed", goal.uuid);
    }
}

/// `T` is the query request content
/// `U` is the query reply content
/// `V` is the additional query that the server can use to calculate the reply
pub fn run_query_server_with_two_supplements<T, U, V, W>(
    mut query_queries: Query<(&mut GoalComponent, &QueryRequest<T>, &mut QueryReply<U>), (With<GoalComponent>, With<QueryRequest<T>>, With<QueryReply<U>>)>,
    supplementary_1_queries: Query<&V, With<V>>,
    supplementary_2_queries: Query<&W, With<W>>,
) where
    T: Send + Sync + 'static,
    U: QueryReplyOpsTriple<T, V, W> + Send + Sync + 'static,
    V: Component,
    W: Component,
{
    for (mut goal, request, mut reply) in query_queries.iter_mut() {
        if goal.is_completed {
            debug!("[{:?}]: Goal is already completed", goal.uuid);
            continue;
        }

        match  U::get_reply(request, &supplementary_1_queries, &supplementary_2_queries) {
            Ok(r) => reply.reply = r,
            Err(_) => {
                error!("[{:?}]: Failed to get reply", goal.uuid);
                continue;
            }
        }

        goal.is_completed = true;
        info!("[{:?}]: Goal is completed", goal.uuid);
    }
}

/// Garbage collection for query requests
/// todo: implement a timeout check
pub fn cleanup_requests(mut commands: Commands, queries: Query<(Entity, &GoalComponent), With<GoalComponent>>) {
    for (entity, goal) in queries.iter() {
        if goal.to_delete {
            commands.entity(entity).despawn();
            info!("[{:?}]: Request despawned", goal.uuid);
        }
    }
}
