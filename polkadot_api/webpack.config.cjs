const path = require('path');

module.exports = {
  entry: './lib.js',
  mode: 'production',
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist')
  },
  module: {
    rules: [{
      test: /\.js$/,
      exclude: /node_modules/,
      use: {
        loader: 'babel-loader',
        options: { presets: ['@babel/preset-env'] }  // ES6 syntax support for webpack config file. 
      }   // end of use object. 

    }] // end of rules array. 

  } // end of module object. 
};