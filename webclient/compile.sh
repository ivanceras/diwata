mkdir -p ../public
elm-make src/Main.elm --yes --output ../public/curtain.js
rsync -vahP --delete index.html ../public/
rsync -vahP --delete app.js ../public
rsync -vahP --delete style.css ../public
rsync -vahP --delete css ../public/
#google-closure-compiler-js ../public/curtain.js > ../public/curtain.min.js
#uglifyjs --compress --mangle -- ../public/curtain.js >  ../public/curtain.uglify.js
