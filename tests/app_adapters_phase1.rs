use yaswitch::adapters::apps::gtk::GtkAdapter;
use yaswitch::adapters::apps::kitty::KittyAdapter;
use yaswitch::adapters::apps::neovim::NeovimAdapter;
use yaswitch::adapters::apps::waybar::WaybarAdapter;
use yaswitch::adapters::contract::validate_adapter_contract;

#[test]
fn kitty_adapter_contract_suite() {
    validate_adapter_contract(&KittyAdapter).expect("expected kitty adapter contract compliance");
}

#[test]
fn neovim_adapter_contract_suite() {
    validate_adapter_contract(&NeovimAdapter).expect("expected neovim adapter contract compliance");
}

#[test]
fn gtk_adapter_contract_suite() {
    validate_adapter_contract(&GtkAdapter).expect("expected gtk adapter contract compliance");
}

#[test]
fn waybar_adapter_contract_suite() {
    validate_adapter_contract(&WaybarAdapter).expect("expected waybar adapter contract compliance");
}
