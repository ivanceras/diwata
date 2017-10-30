function init(){
    app = Elm.Main.fullscreen({
        db_url: "postgres://postgres:p0stgr3s@localhost:5432/mock",
        grouped: true,
    });
}
window.onload = init
