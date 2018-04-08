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
        $('#quiz-pic-'+(i+1)).attr({"src":"./assets/"+(array[i]+startsatone)+".jpg"});
        $('#quiz-pic-'+(i+1)).data( "filename", (array[images_todisp+i]+startsatone));
    }
}



$(document).ready(function() {
    var b = shuffle(Array.from(Array(numberofpics).keys()));
    console.log(b);
    refershimages(b);
    images_selected = b.slice(0,5);
    $.ajax({url: "/training/model_a/"+images_selected.join("/"), 
            success: function(result){
                var a = result.split(";");
                console.log("model a: "+ a);
                sessionStorage.setItem('places1', JSON.stringify(a))
            }});
    $.ajax({url: "/training/model_b/"+images_selected.join("/"), 
            success: function(result){
                var a = result.split(";");
                console.log(a);
                sessionStorage.setItem('places2', JSON.stringify(a));
                initMaps();
//                window.open("./itinerary","_self");
            }});
});

function initMaps() {
    latlongs = [];
    places = JSON.parse(sessionStorage.getItem('places1'));
    console.log(places);
    places.forEach(function(i){
        i = i.split(",")
        latlongs.push({lat: parseFloat(i[0]), lng: parseFloat(i[1])});
    });
    console.log(latlongs);
    markers = [];
    var map1 = new google.maps.Map(document.getElementById('map'), {
      zoom: 12,
      center: {lat:37.91369, lng:145.125}
    });
//    var marker = (new google.maps.Marker({
//      position: {lat:37.91369, lng:145.125},
//      map: map
//    }));
    //    console.log(places[0]);
    //    var markers = [];
    
    latlongs.forEach(function(i){
        markers.push(new google.maps.Marker({
          position: i,
          map: map1
        }));
    });    
//    console.log(markers[0].getPosition());
    var bounds = new google.maps.LatLngBounds();
    for (var i = 0; i < markers.length; i++) {
     bounds.extend(markers[i].getPosition());
    }

    map1.fitBounds(bounds);
    
    
    
    
    
    latlongs = [];
    places = JSON.parse(sessionStorage.getItem('places2'));
    console.log(places);
    places.forEach(function(i){
        i = i.split(",")
        latlongs.push({lat: parseFloat(i[0]), lng: parseFloat(i[1])});
    });
    console.log("latlong"+ latlongs);
    markers = [];
    var map1 = new google.maps.Map(document.getElementById('map2'), {
      zoom: 12,
      center: {lat:37.91369, lng:145.125}
    });
//    var marker = (new google.maps.Marker({
//      position: {lat:37.91369, lng:145.125},
//      map: map
//    }));
    //    console.log(places[0]);
    //    var markers = [];
    
    latlongs.forEach(function(i){
        markers.push(new google.maps.Marker({
          position: i,
          map: map1
        }));
    });    
//    console.log(markers[0].getPosition());
    var bounds = new google.maps.LatLngBounds();
    for (var i = 0; i < markers.length; i++) {
     bounds.extend(markers[i].getPosition());
    }

    map1.fitBounds(bounds);
    
}

function handleA(){
   $.ajax({url: "/training/"+"false", 
                    success: function(result){
                       console.log(result)
                       refresh();
                    }});
}
function handleB(){
    $.ajax({url: "/training/"+"true", 
                    success: function(result){
                        console.log(result)
                        refresh();
                    }});
}

function refresh(){
    location.reload();
}