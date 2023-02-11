const path = require("path");
const fs = require("fs");
const sizeOf = require("image-size");

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

let dimensions = {width: 0, height: 0};
let imageBase64;

try {dimensions = sizeOf(config.imagePath);} catch {imageBase64 = fs.readFileSync(path.join(__dirname, "image.jpeg"), {encoding: "base64"})}

try { imageBase64 = fs.readFileSync(config.imagePath, {encoding: "base64"}); } catch { imageBase64 = fs.readFileSync(path.join(__dirname, "image.jpeg"), {encoding: "base64"}) };

//TODO: fix this
if(!(dimensions.width == 480 && dimensions.height == 480) || dimensions.type != "jpg") {
    imageBase64 = fs.readFileSync(path.join(__dirname, "fallback.jpeg"), {encoding: "base64"});
}

function renderFrame() {
    return imageBase64;
}

function renderPreview(){
    config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
    try {
        imageBase64 = fs.readFileSync(config.imagePath, {encoding: "base64"});
        dimensions = sizeOf(config.imagePath);
        if(!(dimensions.width == 480 && dimensions.height == 480) || dimensions.type != "jpg") {
            imageBase64 = fs.readFileSync(path.join(__dirname, "fallback.jpeg"), {encoding: "base64"});
        }
    } catch {
        imageBase64 = fs.readFileSync(path.join(__dirname, "fallback.jpeg"), {encoding: "base64"});
    }
    return renderFrame();
}

module.exports = {renderFrame, renderPreview, info: {
    title: "Static Image",
    description: "Displays a static image.",
    preview: renderPreview(),
    hasConfig: true,
    controllableParameters: {
        imagePath: {
            type: "file",
            title: "Image",
            defaultValue: path.join(__dirname, "image.jpeg")
        }
    }
}};