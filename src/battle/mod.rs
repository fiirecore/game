pub mod battle;
pub mod battle_manager;
pub mod battle_info;
pub mod battle_context;

pub mod event {
    pub mod battle_event;
}

pub mod transitions {

    pub mod battle_transition_traits;

    pub mod managers {
        pub mod battle_screen_transition_manager;
        pub mod battle_opener_manager;
        pub mod battle_introduction_manager;
        pub mod battle_closer_manager;
    }
    
    pub mod screen_transitions {
        pub mod flash_battle_screen_transition;
        pub mod trainer_battle_screen_transition;
        pub mod vertical_close_battle_screen_transition;
    }
    
    pub mod openers {             
        pub mod trainer_battle_opener;
        pub mod wild_battle_opener;
    }

    pub mod introductions {
        pub mod trainer_battle_introduction;
        pub mod basic_battle_introduction;
        pub mod util {
            pub mod player_intro;
            pub mod intro_text;
        }
    }
    
    pub mod closers {
        pub mod basic_battle_closer;
    }

}