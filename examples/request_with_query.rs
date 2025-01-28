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
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_query_service::*;
use rand::Rng;

#[derive(Component, Clone)]
struct MyAge(i32);

#[derive(Component, Clone, Default, Debug)]
struct FriendAge(i32);

#[derive(Component, Clone, Default, Debug)]
struct TotalAge(i32);

impl QueryReplyOpsDouble<MyAge, FriendAge> for TotalAge {
    fn get_reply(request: &QueryRequest<MyAge>, supplementary_queries: &Query<&FriendAge, With<FriendAge>>) -> Result<Self,()> {
        let mut total_age = request.request.0;
        for friend_age in supplementary_queries.iter() {
            total_age += friend_age.0;
        }
        Ok(TotalAge(total_age))
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins(QueryServicePlugin);

    app.add_event::<QueryEvent<MyAge>>();
    app.add_systems(Update, spawn_request_endpoint::<MyAge, TotalAge>);
    app.add_systems(Update, run_query_server_with_supplement::<MyAge, TotalAge, FriendAge>);
    app.add_systems(Update, cleanup_requests);

    app.add_plugins(EguiPlugin);
    app.add_systems(Update, interaction_panel);
    app.add_systems(Update, request_panel);
    app.run();
}

fn interaction_panel(mut commands: Commands, mut contexts: EguiContexts, mut query_event_writer: EventWriter<QueryEvent<MyAge>>, queries: Query<&FriendAge, With<FriendAge>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Interaction");
            ui.horizontal(|ui| {
                if ui.button("Send query request").clicked() {
                    query_event_writer.send(QueryEvent {
                        uuid: uuid::Uuid::new_v4(),
                        request: MyAge(22),
                    });
                }
            });
            ui.separator();

            ui.label("Friends");
            for friend_age in queries.iter() {
                ui.horizontal(|ui| {
                    ui.label(format!("Friend: {:?}", friend_age));
                });
            }

            if ui.button("Add friend").clicked() {
                commands.spawn(FriendAge(rand::rng().random_range(18..100)));
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn request_panel(mut contexts: EguiContexts, mut reply_queries: Query<(&mut GoalComponent, &QueryReply<TotalAge>), With<QueryReply<TotalAge>>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Requests");
            for (mut goal, query) in reply_queries.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("Total age: {:?}", query.reply.0));
                    if ui.button("Delete").clicked() {
                        info!("Deleting request...");
                        goal.to_delete = true;
                    }
                });
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
