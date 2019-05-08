use sauron::Callback;

pub struct Cmd<MSG>(pub Vec<Callback<(), MSG>>);

impl<MSG> Cmd<MSG> {
    pub fn new<F>(cmd: F) -> Self
    where
        F: Fn(()) -> MSG + 'static,
    {
        let cb: Callback<(), MSG> = cmd.into();
        Cmd(vec![cb])
    }

    pub fn batch<F>(cmd: Vec<F>) -> Self
    where
        F: Fn(()) -> MSG + 'static,
    {
        let cmd_vec: Vec<Callback<(), MSG>> = cmd.into_iter().map(Into::into).collect();
        Cmd(cmd_vec)
    }
}
