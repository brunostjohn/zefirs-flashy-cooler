const path = require("path");
const fs = require("fs");

let config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));

let imageBase64 = fs.readFileSync(config.imagePath, {encoding: "base64"});

function renderFrame() {
    return imageBase64;
}

function renderPreview(){
    config = JSON.parse(fs.readFileSync(path.join(__dirname, "config.json")));
    imageBase64 = fs.readFileSync(config.imagePath, {encoding: "base64"});
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