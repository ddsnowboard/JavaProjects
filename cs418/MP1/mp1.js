"use strict";
/**
 * @file A simple WebGL example drawing a circle
 * @author Eric Shaffer <shaffer1@illinois.edu>  
 */

/** @global The WebGL context */
var gl;

/** @global The HTML5 canvas we draw on */
var canvas;

/** @global A simple GLSL shader program */
var shaderProgram;

/** @global The WebGL buffer holding the triangle */
var vertexPositionBuffer;

/** @global The WebGL buffer holding the vertex colors */
var vertexColorBuffer;

/** @global The Modelview matrix */
var mvMatrix = mat4.create();

/** @global The Projection matrix */
var pMatrix = mat4.create();

/** @global The angle of rotation around the x axis */
var defAngle = 0;

let vertexShader;

let fragmentShader;

//----------------------------------------------------------------------------------
/**
 * Sends projection/modelview matrices to shader
 */
function setMatrixUniforms() {
    gl.uniformMatrix4fv(shaderProgram.mvMatrixUniform, false, mvMatrix);
    gl.uniformMatrix4fv(shaderProgram.pMatrixUniform, false, pMatrix);
}

/**
 * Translates degrees to radians
 * @param {Number} degrees Degree input to function
 * @return {Number} The radians that correspond to the degree input
 */
function degToRad(degrees) {
    return degrees * Math.PI / 180;
}

/**
 * Translates arr by {x,y,z}
 */
function translate(arr, {x,y,z}) {
    return arr.map((el, idx) => {
        switch(idx % 3) {
            case 0: // x
                return el + x;
            case 1: // y 
                return el + y;
            case 2: // z
                return el + z
        }
    });
}

/**
 * Makes an array [0,1,2,3,..n - 1]
 */
function range(n) {
    let out = [];
    for(let i = 0; i < n; i++)
        out.push(i);
    return out;
}

/**
 * Creates a context for WebGL
 * @param {element} canvas WebGL canvas
 * @return {Object} WebGL context
 */
function createGLContext(canvas) {
    var names = ["webgl", "experimental-webgl"];
    var context = null;
    for (var i=0; i < names.length; i++) {
        try {
            context = canvas.getContext(names[i]);
        } catch(e) {}
        if (context) {
            break;
        }
    }
    if (context) {
        context.viewportWidth = canvas.width;
        context.viewportHeight = canvas.height;
    } else {
        alert("Failed to create WebGL context!");
    }
    return context;
}

/**
 * Loads Shaders
 * @param {string} id ID string for shader to load. Either vertex shader/fragment shader
 */
function loadShaderFromDOM(id) {
    var shaderScript = document.getElementById(id);

    // If we don't find an element with the specified id
    // we do an early exit 
    if (!shaderScript) {
        return null;
    }

    // Loop through the children for the found DOM element and
    // build up the shader source code as a string
    var shaderSource = "";
    var currentChild = shaderScript.firstChild;
    while (currentChild) {
        if (currentChild.nodeType == 3) { // 3 corresponds to TEXT_NODE
            shaderSource += currentChild.textContent;
        }
        currentChild = currentChild.nextSibling;
    }

    var shader;
    if (shaderScript.type == "x-shader/x-fragment") {
        shader = gl.createShader(gl.FRAGMENT_SHADER);
    } else if (shaderScript.type == "x-shader/x-vertex") {
        shader = gl.createShader(gl.VERTEX_SHADER);
    } else {
        return null;
    }

    gl.shaderSource(shader, shaderSource);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        alert(gl.getShaderInfoLog(shader));
        return null;
    } 
    return shader;
}

/**
 * Setup the fragment and vertex shaders
 */
function setupShaders() {
    vertexShader = loadShaderFromDOM("shader-vs");
    fragmentShader = loadShaderFromDOM("shader-fs");

    shaderProgram = gl.createProgram();
    gl.attachShader(shaderProgram, vertexShader);
    gl.attachShader(shaderProgram, fragmentShader);
    gl.linkProgram(shaderProgram);

    if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
        alert("Failed to setup shaders");
    }

    gl.useProgram(shaderProgram);
    shaderProgram.vertexPositionAttribute = gl.getAttribLocation(shaderProgram, "aVertexPosition");
    gl.enableVertexAttribArray(shaderProgram.vertexPositionAttribute);

    shaderProgram.vertexColorAttribute = gl.getAttribLocation(shaderProgram, "aVertexColor");
    gl.enableVertexAttribArray(shaderProgram.vertexColorAttribute);
    shaderProgram.mvMatrixUniform = gl.getUniformLocation(shaderProgram, "uMVMatrix");
    shaderProgram.pMatrixUniform = gl.getUniformLocation(shaderProgram, "uPMatrix");
}

/**
 * Populate vertex buffer with data
  @param {number} number of vertices to use around the circle boundary
  */
function loadVertices() {
    console.log("Frame", defAngle);
    //Generate the vertex positions    
    vertexPositionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexPositionBuffer);

    // Start with vertex at the origin    
    let topStripe = [
        -5.0,3.5,0.0,
        -4.2,3.5,0.0,
        -5.0,5.0,0.0,

        -4.2,3.5,0.0,
        -5.0,5.0,0.0,
        -2.0,3.5,0,

        -5.0,5.0,0.0,
        -2.0,3.5,0,
        5,5,0,

        -2.0,3.5,0,
        5,5,0,
        2,3.5,0,

        5,5,0,
        2,3.5,0,
        4.2,3.5,0,

        2,3.5,0,
        4.2,3.5,0,
        5,3.5,0,

        4.2,3.5,0,
        5,3.5,0,
        5,5,0
    ];

    let leftSide = [
        -4.2,3.5,0,
        -2,3.5,0,
        -4.2,2.05,0,

        -2,3.5,0,
        -4.2,2.05,0,
        -2,2.05,0,

        -2,2.05,0,
        -2,-2.05,0,
        -4.2,2.05,0,

        -4.2,2.05,0,
        -2,-2.05,0,
        -4.2,-2.05,0,

        -2, 2.05,0,
        -1.15,2.05,0,
        -1.15,-2.05,0,

        -1.15,-2.05,0,
        -2, 2.05,0,
        -2, -2.05,0,

        -4.2,-3.5,0,
        -2,-3.5,0,
        -4.2,-2.05,0,

        -2,-3.5,0,
        -4.2,-2.05,0,
        -2,-2.05,0,

        -2,-2.05,0,
        -2,2.05,0,
        -4.2,-2.05,0,
    ];
    
    let rightSide = leftSide.map((el, idx) => idx % 3 === 0 ? -el : el);
    const TOP_OF_STRIPES = -3.8;
    const STRIPE_WIDTH = 8.4 / 11;
    const LEFT_OF_STRIPES = -4.2;
    let leftStripe = [
        LEFT_OF_STRIPES, TOP_OF_STRIPES, 0,
        LEFT_OF_STRIPES,-4.6,0,
        LEFT_OF_STRIPES + STRIPE_WIDTH,TOP_OF_STRIPES,0,

        LEFT_OF_STRIPES + STRIPE_WIDTH, TOP_OF_STRIPES, 0,
        LEFT_OF_STRIPES,-4.6,0,
        LEFT_OF_STRIPES + STRIPE_WIDTH, -5.0, 0
    ];

    let rightStripe = leftStripe.map((el, idx) => idx % 3 == 0 ? -el : el);
    const NUM_STRIPES = 6;
    const INTERSTRIPE_DISTANCE = 8.4 / 5.5;
    const M = -.85; // The slope of the line made by the bottom of the stripes
    let stripes = range(NUM_STRIPES).map((n) => {
        let out;
        if(n >= 3) {
            out = translate(rightStripe, {x:-INTERSTRIPE_DISTANCE * (5 - n), y: 0, z: 0});
        }
        else {
            out = translate(leftStripe, {x:INTERSTRIPE_DISTANCE * n, y: 0, z: 0});
        }
        out = out.map((el, idx) => {
            if(idx % 3 == 1 && el !== TOP_OF_STRIPES) { // is y coord
                let coefficient = (n >= 3 ? (5 - n) : n);
                return el + coefficient * M;
            } else {
                return el;
            }
        });
        return out;
    }).flat();
    let orangeVertices = stripes;

    let allVertices = topStripe.concat(leftSide).concat(rightSide).concat(orangeVertices);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(allVertices), gl.DYNAMIC_DRAW);
    vertexPositionBuffer.itemSize = 3;
    vertexPositionBuffer.numberOfItems = allVertices.length / 3;
    // debugger;
    return {blue: topStripe.length / 3 + leftSide.length * 2 / 3, orange: orangeVertices.length / 3};
}

/**
 * Populate color buffer with data
  @param {number} number of vertices to use around the circle boundary
  */
function loadColors(counts) {
    vertexColorBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexColorBuffer);

    let orange = {r: .91, g: .29, b: .21};
    let blue = {r: 0.09, g: 0.16, b: 0.3};
    var a=1.0;
    let colors = [];
    for (let i=0; i<counts.blue; i++){
        /*
        let randRed = Math.random();
        let randBlue = Math.random();
        let randGreen = Math.random();
        colors.push(randRed);
        colors.push(randBlue);
        colors.push(randGreen);
        colors.push(1);
        colors.push(randRed);
        colors.push(randBlue);
        colors.push(randGreen);
        colors.push(1);
        colors.push(randRed);
        colors.push(randBlue);
        colors.push(randGreen);
        colors.push(1);
        i+= 2;
        */
        let {r, g, b} = blue;
        colors.push(r);
        colors.push(g);
        colors.push(b);
        colors.push(a);
    }

    for(let i = 0; i < counts.orange; i++) {
        let {r, g, b} = orange;
        colors.push(r);
        colors.push(g);
        colors.push(b);
        colors.push(a);
    }

    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW);
    vertexColorBuffer.itemSize = 4;
    vertexColorBuffer.numItems = counts.orange + counts.blue;  
}
/**
 * Populate buffers with data
   @param {number} number of vertices to use around the circle boundary
   */
function setupBuffers() {

    //Generate the vertex positions    
    let counts = loadVertices();

    //Generate the vertex colors
    loadColors(counts);
}

/**
 * Draw call that applies matrix transformations to model and draws model in frame
 */
let ticks;
function draw() { 
    if(ticks === undefined)
        ticks = 0;
    gl.viewport(0, 0, gl.viewportWidth, gl.viewportHeight);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT); 

    mat4.identity(mvMatrix);
    mat4.rotateZ(mvMatrix, mvMatrix, 0.01 * ticks);
    
    mat4.ortho(pMatrix,-7,7,-9,9,5,-5);  


    gl.bindBuffer(gl.ARRAY_BUFFER, vertexPositionBuffer);
    gl.vertexAttribPointer(shaderProgram.vertexPositionAttribute, 
        vertexPositionBuffer.itemSize, gl.FLOAT, false, 0, 0);
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexColorBuffer);
    gl.vertexAttribPointer(shaderProgram.vertexColorAttribute, 
        vertexColorBuffer.itemSize, gl.FLOAT, false, 0, 0);

    setMatrixUniforms();
    gl.drawArrays(gl.TRIANGLES, 0, vertexPositionBuffer.numberOfItems);
    ticks++;
}


/**
 * Startup function called from html code to start program.
 */
function startup() {
    canvas = document.getElementById("myGLCanvas");
    gl = createGLContext(canvas);
    setupShaders(); 
    setupBuffers();
    gl.clearColor(1.0, 1.0, 1.0, 1.0);
    gl.enable(gl.DEPTH_TEST);
    tick();
}

/**
 * Tick called for every animation frame.
 */
function tick() {
    requestAnimFrame(tick);
    draw();
}

function deformSin(x, y, angle) {
    let pt = vec2.fromValues(x, y);
    let dist = 0.2 * Math.sin(angle + degToRad(defAngle));
    vec2.normalize(pt, pt);
    vec2.scale(pt, pt, dist);
    return pt;
}
