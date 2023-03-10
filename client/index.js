
let query = document.getElementById("input");

query.addEventListener("keypress", (e) => {
    
    if (e.key == "Enter"){
        // post req
        console.log("hello");
    }
    
    if (e.key == "R" || e.key == "r"){
        let cursorPos=document.selection.createRange().duplicate();
        let clickx = cursorPos.getBoundingClientRect().left; 
        let clicky = cursorPos.getBoundingClientRect().top;
        // console.log(query.innerHTML.slice(0, query.innerHTML.length-1));
        // query.innerHTML = query.innerHTML.slice(0, query.innerHTML.length-1);
        query.innerHTML.length--;
        query.innerHTML += '<span style="color: red;">'+e.key+'</span>';
        
        cursorPos = document.body.createTextRange();
        cursorPos.moveToPoint(clickx, clicky);
        cursorPos.select();

        console.log(query.innerHTML);
    } else if (e.key == "G" || e.key == "g"){
        
    } 
})
