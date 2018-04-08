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
    for(i=0;i<=4;i++){
        $('#quiz-pic-'+(i+1)).attr({"src":"./assets/"+(array[i]+startsatone)+".png"});
        $('#quiz-pic-'+(i+1)).data( "filename", (array[images_todisp+i]+startsatone));
    }
}
function initMap() {
        var uluru = {lat: -25.363, lng: 131.044};
        var map = new google.maps.Map(document.getElementById('map'), {
          zoom: 4,
          center: uluru
        });
        var marker = new google.maps.Marker({
          position: uluru,
          map: map
        });
      }
function handleA(){
   $.ajax({url: "/training/"+"false", 
                    success: function(result){
                       console.log(result)
                    }});
}
function handleB(){
    $.ajax({url: "/training/"+"true", 
                    success: function(result){
                        console.log(result)
                    }});
}



$(document).ready(function() {
    var images_selected = [];
    var b = shuffle(Array.from(Array(numberofpics).keys()));
    console.log(b);
    refershimages(b);
    
});