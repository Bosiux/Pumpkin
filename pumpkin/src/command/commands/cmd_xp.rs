use std::sync::Arc;

use crate::command::args::FindArg;
use async_trait::async_trait;
use pumpkin_core::text::{color::NamedColor, TextComponent};



use crate::{
    command::{
        args::{
            arg_bounded_num::BoundedNumArgumentConsumer, arg_players::PlayersArgumentConsumer,
            ConsumedArgs,
        },
        tree::CommandTree,
        tree_builder::{argument, literal},
        CommandError, CommandExecutor, CommandSender,
    },
    entity::player::Player,
    server::Server,
};

const NAMES: [&str; 2] = ["xp", "experience"];
const DESCRIPTION: &str = "Adds, sets or queries player experience";
const XP_PER_LEVEL: i32 = 17;

const ARG_TARGET: &str = "targets";
const ARG_AMOUNT: &str = "amount";

struct XpAddExecutor(bool); // true for levels, false for points

#[async_trait]
impl CommandExecutor for XpAddExecutor {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender<'a>,
        _server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let targets = PlayersArgumentConsumer::find_arg(args, ARG_TARGET)?;
        let amount = BoundedNumArgumentConsumer::<i32>::find_arg(args, ARG_AMOUNT)?; 

        for target in targets {
            let points = if self.0 {
                amount.unwrap() * XP_PER_LEVEL
            } else {
                amount.unwrap()
            };
            // CANT BE AWAITED
            target.add_experience(points);
            notify_xp_change(target.clone(), amount.expect("REASON"), self.0);
            
        }

        Ok(())
    }
}

struct XpSetExecutor(bool); // true for levels, false for points

#[async_trait]
impl CommandExecutor for XpSetExecutor {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender<'a>,
        _server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let targets = PlayersArgumentConsumer::find_arg(args, ARG_TARGET)?;
        let amount = BoundedNumArgumentConsumer::<i32>::find_arg(args, ARG_AMOUNT)?;

        for target in targets {
            let points = if self.0 {
                amount.unwrap() * XP_PER_LEVEL
            } else {
                amount.unwrap()
            };

            // CANT BE AWAITED
            target.set_experience(points);
            notify_xp_set(target.clone(), amount.expect("REASON"), self.0);
            
        }

        Ok(())
    }
}

struct XpQueryExecutor(bool); // true for levels, false for points

#[async_trait]
impl CommandExecutor for XpQueryExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let targets = PlayersArgumentConsumer::find_arg(args, ARG_TARGET)?;

        for target in targets {
            let xp = if self.0 {
                target.get_experience() / XP_PER_LEVEL 
            } else {
                target.get_experience() 
            };

            let unit = if self.0 { "levels" } else { "points" };
            let message = TextComponent::text(format!(
                "{} has {} {} of experience",
                target.gameprofile.name, xp, unit
            ))
            .color_named(NamedColor::Green);

            sender.send_message(message).await;
        }

        Ok(())
    }
}

async fn notify_xp_change(target: Arc<Player>, amount: i32, is_levels: bool) {
    let unit = if is_levels { "levels" } else { "points" };
    let message = TextComponent::text(format!("Added {} {} of experience", amount, unit))
        .color_named(NamedColor::Green);
    target.send_system_message(&message).await;
}

async fn notify_xp_set(target: Arc<Player>, amount: i32, is_levels: bool) {
    let unit = if is_levels { "levels" } else { "points" };
    let message = TextComponent::text(format!("Set experience to {} {}", amount, unit))
        .color_named(NamedColor::Green);
    target.send_system_message(&message).await;
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .with_child(
            literal("add").with_child(
                argument(ARG_TARGET, PlayersArgumentConsumer).with_child(
                    argument(ARG_AMOUNT, BoundedNumArgumentConsumer::<i32>::new())
                        .with_child(literal("levels").execute(XpAddExecutor(true)))
                        .with_child(literal("points").execute(XpAddExecutor(false)))
                        .execute(XpAddExecutor(false)),
                ),
            ),
        )
        .with_child(
            literal("set").with_child(
                argument(ARG_TARGET, PlayersArgumentConsumer).with_child(
                    argument(ARG_AMOUNT, BoundedNumArgumentConsumer::<i32>::new())
                        .with_child(literal("levels").execute(XpSetExecutor(true)))
                        .with_child(literal("points").execute(XpSetExecutor(false)))
                        .execute(XpSetExecutor(false)),
                ),
            ),
        )
        .with_child(
            literal("query").with_child(
                argument(ARG_TARGET, PlayersArgumentConsumer)
                    .with_child(literal("levels").execute(XpQueryExecutor(true)))
                    .with_child(literal("points").execute(XpQueryExecutor(false)))
                    .execute(XpQueryExecutor(false)),
            ),
        )
}
