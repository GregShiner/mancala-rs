use crate::game::{Game, PlayerSide};
use crate::solver::SequenceTree;

use cursive::{Cursive, CursiveExt};
use cursive::views::{TextView};
use cursive_core::theme::{BaseColor::*, Color::*, PaletteColor::*};
use cursive_core::view::Nameable;

pub mod game;
pub mod solver;

fn main() {
    unsafe {
        println!("{:?}", *std::ptr::null::<i32>()); 
    }
    let game = Game::new();
    //let mut tree = SequenceTree::new(game.clone());
    //tree.generate_tree(PlayerSide::Player, PlayerSide::Player, 0);

    let mut siv = Cursive::default();
    // disable shadows, set background to black, and text to white
    siv.update_theme(|theme| {
        theme.shadow = false;
        theme.palette[Background] = Dark(Black);
        theme.palette[View] = Dark(Black);
        theme.palette[Primary] = Light(White);
    });
    // Main menu has the following options:
    // - Reset Game
    // - Manually Enter Board State
    // - Stash Game
    // - Load Game
    // - Test move (displays resulting board)
    // - Play move
    // - Generate Sequence Tree
    // - Solve Game (not yet implemented)

    siv.add_layer(TextView::new(format!("{:?}", game)).with_name("board_text"));

    let mut main_menu = cursive::views::LinearLayout::vertical();
    main_menu.add_child(TextView::new("Main Menu").with_name("main_menu_title"));
    main_menu.add_child(TextView::new("(R)eset Game").with_name("reset_game"));
    main_menu.add_child(TextView::new("(M)anually Enter Board State").with_name("enter_board_state"));
    main_menu.add_child(TextView::new("(S)tash Game").with_name("stash_game"));
    main_menu.add_child(TextView::new("(L)oad Game").with_name("load_game"));
    main_menu.add_child(TextView::new("(T)est Move").with_name("test_move"));
    main_menu.add_child(TextView::new("(P)lay Move").with_name("play_move"));
    main_menu.add_child(TextView::new("(G)enerate Sequence Tree").with_name("generate_sequence_tree"));
    main_menu.add_child(TextView::new("(F)ind best move").with_name("solve_game"));
    siv.add_layer(main_menu.with_name("main_menu"));

    // add menu callbacks
    siv.add_global_callback('r', move |s| {
        s.call_on_name("board_text", |view: &mut TextView| {
            view.set_content(format!("{:?}", game));
        });
    });

    siv.add_global_callback('m', |s| {
        let game = Game::new();
        s.call_on_name("board_text", |view: &mut TextView| {
            view.set_content(format!("{:?}", game));
        });
    });


    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
