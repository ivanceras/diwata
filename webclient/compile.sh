mkdir -p ../public
elm-make src/Main.elm --yes --output ../public/diwata.js
rsync -vahP --delete index.html ../public/
rsync -vahP --delete app.js ../public
rsync -vahP --delete style.css ../public
rsync -vahP --delete css ../public/
#google-closure-compiler-js ../public/diwata.js > ../public/diwata.min.js
