#[macro_export]
macro_rules! close_menu {
    ( $( $x:expr ),* ) => {
        Box::new(|menu_man: &mut MenuManager| {
            menu_man.close_menu();
        })
    };
}
#[macro_export]
macro_rules! open_menu {
    ( $x:expr ) => {
        Box::new(move |menu_man: &mut MenuManager| {
            menu_man.open_menu($x.into());
        })
    };
}