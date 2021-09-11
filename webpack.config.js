const webpack = require('webpack');
const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const { VueLoaderPlugin } = require('vue-loader');

const config = {
    context: path.resolve(__dirname, 'playday_web/static/src'),
    entry: './app.js',
    output: {
        path: path.resolve(__dirname, 'playday_web/static/dist'),
        filename: 'bundle.js',
    },
    resolve: {
        extensions: ['.js', '.jsx', '.css'],
        alias: { vue: '@vue/runtime-dom' }
    },
    plugins: [
        new webpack.ProgressPlugin(),
        new CleanWebpackPlugin(),
        new MiniCssExtractPlugin(),
        new VueLoaderPlugin(),
    ],
    devtool: 'source-map',
    module: {
        rules: [
            {
                test: /\.vue$/,
                loader: 'vue-loader'
            },
            {
                test: /\.css$/i,
                use: [MiniCssExtractPlugin.loader, 'css-loader', 'postcss-loader'],
            },
            {
                test: /\.svg$/,
                loader: 'svg-inline-loader'
            },
        ],
    },
    watchOptions: {
        ignored: ['target/', '**/*.rs']
    },
};

module.exports = config;
