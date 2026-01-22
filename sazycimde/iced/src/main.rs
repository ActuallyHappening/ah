use ah_sazycimde_iced::toplevel::TopLevelState;

fn main() -> iced::Result {
	iced::application(TopLevelState::default, TopLevelState::update, TopLevelState::view).theme(TopLevelState::theme).run()
}

