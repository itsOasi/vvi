import {CanvasPainter, randomStrokeData} from "https://cdn.jsdelivr.net/gh/itsOasi/canvas_painter/canvasPainter.js";

import init, {test} from "./pkg/vvi.js"
console.log("hello world");
// todo: replace randomStrokeData with fn to get data from nebulizer
// let cp = new CanvasPainter("canvas", randomStrokeData);
// cp.play();
await init()
console.log(test(1, 2));