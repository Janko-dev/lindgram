
const btn = document.getElementById("generate_btn");
const axiom = document.getElementById("axiom_input");
const rules = document.getElementById("rules_input");
const angle = document.getElementById("angle");
const line = document.getElementById("line");
const n = document.getElementById("n");
const offset = document.getElementById("offset");

const canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

const WIDTH  = 500;
const HEIGHT = 500;

ctx.width = WIDTH;
ctx.height = HEIGHT;

ctx.fillStyle = "white"
ctx.fillRect(0, 0, WIDTH, HEIGHT);

let offsetX = WIDTH/2;
let offsetY = HEIGHT/2;

offset.innerText = `Offset = (${offsetX}, ${offsetY})\nClick on the canvas to position the offset`

canvas.addEventListener("mousedown", (e) => {
    // console.log(e)
    offsetX = e.offsetX;
    offsetY = e.offsetY;
    offset.innerText = `Offset = (${offsetX}, ${offsetY})\nClick on the canvas to position the offset`
})

btn.addEventListener("click", (e) => {
    // console.log(axiom.innerText);
    // console.log();
    fetch("/generate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            "axiom": axiom.innerText,
            "rules": rules.innerText,
            "n": parseInt(n.value),
            "angle": parseInt(angle.innerText),
            "line_len": parseInt(line.innerText),
            "width": WIDTH,
            "height": HEIGHT,
            "offsetX": offsetX,
            "offsetY": offsetY
        }),
    }).then(res => {
        return res.arrayBuffer();
    }).then(res => {
        let pixels = new Uint8ClampedArray(res.slice(0, WIDTH*HEIGHT*4))
        const imageData = new ImageData(pixels, WIDTH, HEIGHT);
        // console.log(imageData.data.filter((v) => v == 0))
        ctx.putImageData(imageData, 0, 0);
    })
})

// query.addEventListener("keypress", (e) => {
    
//     if (e.key == "Enter"){
//         // post req
//         console.log("hello");
//     }
    
//     if (e.key == "R" || e.key == "r"){
//         let cursorPos=document.selection.createRange().duplicate();
//         let clickx = cursorPos.getBoundingClientRect().left; 
//         let clicky = cursorPos.getBoundingClientRect().top;
//         // console.log(query.innerHTML.slice(0, query.innerHTML.length-1));
//         // query.innerHTML = query.innerHTML.slice(0, query.innerHTML.length-1);
//         query.innerHTML.length--;
//         query.innerHTML += '<span style="color: red;">'+e.key+'</span>';
        
//         cursorPos = document.body.createTextRange();
//         cursorPos.moveToPoint(clickx, clicky);
//         cursorPos.select();

//         console.log(query.innerHTML);
//     } else if (e.key == "G" || e.key == "g"){
        
//     } 
// })
