var numberofpics = 20;
var startsatone = 0;

var images_todisp = 0;
function shuffle(a) {
    var j, x, i;
    for (i = a.length - 1; i > 0; i--) {
        j = Math.floor(Math.random() * (i + 1));
        x = a[i];
        a[i] = a[j];
        a[j] = x;
    }
    return a;
}
function refershimages(array){
    for(i=0;i<=3;i++){
        $('#quiz-pic-'+(i+1)).attr({"src":"./assets/"+(array[images_todisp+i]+startsatone)+".png"});
        $('#quiz-pic-'+(i+1)).data( "filename", (array[images_todisp+i]+startsatone));
    }
}



$(document).ready(function() {
    var images_selected = [];
    var b = shuffle(Array.from(Array(numberofpics).keys()));
    console.log(b);
    refershimages(b);
    $('.quiz-pic').on('click',function(){
        console.log($(this).data("filename"));
        console.log(images_todisp + 4);
        images_selected.push($(this).data("filename"));
        if (images_todisp + 4 >= b.length){
            $('.quiz-pic').attr("src","./assets/loading.gif");
            $('.quiz-pic').off('click');
            $.ajax({url: "/results/"+images_selected.join("/"), 
                    success: function(result){
                        var a = result.split(",");
                        console.log(a);
                        // window.open("./itinerary","_self")
                        sessionStorage.setItem('places', JSON.stringify(a))
                    }});
            
        }else{
            images_todisp += 4;
            refershimages(b);
        }
    });
});