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

#[derive(Component, Clone)]
struct Apple(i32);

#[derive(Component, Clone)]
struct Banana(i32);

#[derive(Component, Clone)]
struct Orange(i32);

#[derive(Component, Clone)]
struct Request(Fruit);

#[derive(Clone)]
enum Fruit {
    Apple,
    Banana,
    Orange,
}

#[derive(Component, Clone, Default, Debug)]
struct Reply(i32);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins(QueryServicePlugin);

    app.add_systems(Startup, spawn_fruits);
    app.add_systems(Update, cleanup_requests);
    app.add_event::<QueryEvent<Request>>();
    app.add_systems(Update, spawn_request_endpoint::<Request, Reply>);
    app.add_systems(Update, run_query_server::<Request, Reply>);

    app.add_plugins(EguiPlugin);
    app.add_systems(Update, interaction_panel);
    app.add_systems(Update, request_panel);
    app.run();
}

fn spawn_fruits(mut commands: Commands) {
    commands.spawn(Apple(1));
    commands.spawn((Apple(1), Banana(2)));
    commands.spawn((Apple(1), Banana(2), Orange(3)));
}

impl QueryReplyOps<Request> for Reply {
    fn get_reply(world: &mut World, request: &QueryRequest<Request>) -> Reply {
        match request.request.0 {
            Fruit::Apple => {
                let mut apples = world.query_filtered::<&Apple, With<Apple>>();
                let mut count = 0;
                for apple in apples.iter(world) {
                    count = count + apple.0;
                }
                return Reply(count as i32);
            }
            Fruit::Banana => {
                let mut bananas = world.query_filtered::<&Banana, With<Banana>>();
                let mut count = 0;
                for banana in bananas.iter(world) {
                    count = count + banana.0;
                }
                return Reply(count as i32);
            }
            Fruit::Orange => {
                let mut oranges = world.query_filtered::<&Orange, With<Orange>>();
                let mut count = 0;
                for orange in oranges.iter(world) {
                    count = count + orange.0;
                }
                return Reply(count as i32);
            }
        }
    }
}

fn interaction_panel(mut contexts: EguiContexts, mut query_event_writer: EventWriter<QueryEvent<Request>>, queries: Query<&Reply, With<Reply>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Interaction");
            ui.horizontal(|ui| {
                if ui.button("Send apple request").clicked() {
                    query_event_writer.send(QueryEvent {
                        uuid: uuid::Uuid::new_v4(),
                        request: Request(Fruit::Apple),
                    });
                }
                if ui.button("Send banana request").clicked() {
                    query_event_writer.send(QueryEvent {
                        uuid: uuid::Uuid::new_v4(),
                        request: Request(Fruit::Banana),
                    });
                }
                if ui.button("Send oranage request").clicked() {
                    query_event_writer.send(QueryEvent {
                        uuid: uuid::Uuid::new_v4(),
                        request: Request(Fruit::Orange),
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

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn request_panel(mut contexts: EguiContexts, mut reply_queries: Query<(&mut GoalComponent, &QueryReply<Reply>), With<QueryReply<Reply>>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Requests");
            for (mut goal, query) in reply_queries.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("Fruit count: {:?}", query.reply.0));
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
