mkdir -p ../public
elm-make src/Main.elm --yes --output ../public/diwata.js
rsync -vahP --delete index.html ../public/
rsync -vahP --delete app.js ../public
rsync -vahP --delete style.css ../public
rsync -vahP --delete icon.css ../public
rsync -vahP --delete fonts ../public/
rsync -vahP --delete css ../public/
rsync -vahP --delete img ../public/
