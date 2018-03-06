./compile.sh
# this needs gogle-closure-compiler, install via `npm install google-closure-compiler`
google-closure-compiler-js ../public/diwata.js > ../public/diwata.min.js
sed -i -- 's/diwata.js/diwata.min.js/g' ../public/index.html 
