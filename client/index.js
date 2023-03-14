
const btn = document.getElementById("generate_btn");
const axiom = document.getElementById("axiom_input");
const rules = document.getElementById("rules_input");

btn.addEventListener("click", (e) => {
    console.log(axiom.innerText);
    console.log(rules.innerText);
    fetch("/generate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            "axiom": axiom.innerText,
            "rules": rules.innerText,
            "n": 2,
        }),
    }).then(res => {
        console.log(res);
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
