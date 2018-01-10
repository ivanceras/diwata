// the db_url and grouped is retrieved from the indexed db
function init(){
    app = Elm.Main.fullscreen(
        { db_url: "postgres://postgres:p0stgr3s@localhost:5432/mock",
          api_endpoint: "http://localhost:8000",
          grouped: true,
        }
    );
}
window.onload = init
