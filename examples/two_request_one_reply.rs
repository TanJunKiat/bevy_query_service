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
struct Question1;

#[derive(Component, Clone)]
struct Question2;

#[derive(Component, Clone, Default, Debug)]
struct Answer(String);

impl QueryReplyOps<Question1> for Answer {
    fn get_reply(_world: &mut World, _request: &QueryRequest<Question1>) -> Self {
        Answer("My answer to question 1.".into())
    }
}

impl QueryReplyOps<Question2> for Answer {
    fn get_reply(_world: &mut World, _request: &QueryRequest<Question2>) -> Self {
        Answer("My answer to question 2.".into())
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins(QueryServicePlugin);

    app.add_event::<QueryEvent<Question1>>();
    app.add_event::<QueryEvent<Question2>>();
    app.add_systems(Update, spawn_request_endpoint::<Question1, Answer>);
    app.add_systems(Update, spawn_request_endpoint::<Question2, Answer>);
    app.add_systems(Update, run_query_server::<Question1, Answer>);
    app.add_systems(Update, run_query_server::<Question2, Answer>);

    app.add_plugins(EguiPlugin);
    app.add_systems(Update, interaction_panel);
    app.add_systems(Update, request_panel);
    app.run();
}

fn interaction_panel(mut contexts: EguiContexts, mut question_1_event: EventWriter<QueryEvent<Question1>>, mut question_2_event: EventWriter<QueryEvent<Question2>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Interaction");
            ui.horizontal(|ui| {
                if ui.button("Asking question 1").clicked() {
                    question_1_event.send(QueryEvent {
                        uuid: uuid::Uuid::new_v4(),
                        request: Question1,
                    });
                }
                if ui.button("Asking question 2").clicked() {
                    question_2_event.send(QueryEvent {
                        uuid: uuid::Uuid::new_v4(),
                        request: Question2,
                    });
                }
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn request_panel(mut contexts: EguiContexts, reply_queries: Query<&QueryReply<Answer>, With<QueryReply<Answer>>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Requests");
            for query in reply_queries.iter() {
                ui.label(format!("Request: {:?}", query.reply.0));
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
