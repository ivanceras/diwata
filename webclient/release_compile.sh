./compile.sh

cd ../public

google-closure-compiler-js diwata.js > diwata.min.js
find . -name 'index.html' -type f -exec sed -i 's/diwata.js/diwata.min.js/g' {} +

cd ..
