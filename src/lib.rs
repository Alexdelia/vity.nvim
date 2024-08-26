use nvim_oxi::api::{self, opts::*, types::*, Window};
use nvim_oxi::{print, Dictionary, Function};

#[nvim_oxi::plugin]
fn vity() -> nvim_oxi::Result<Dictionary> {
    let load: Function<(), Result<(), api::Error>> = Function::from_fn(move |()| {
        print!("vity loaded"); // debug purpose

        Ok(())
    });

    // avoid `load.clone()` and gives possible plugin extension
    let setup: &Function<(), Result<(), api::Error>> = &load;

    // avoid `load.clone()`
    let colorscheme: &Function<(), Result<(), api::Error>> = &load;

    Ok(Dictionary::from_iter([
        ("load", load),
        ("setup", setup),
        ("colorscheme", colorscheme),
    ]))
}
