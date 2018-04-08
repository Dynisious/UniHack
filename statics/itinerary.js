function initMap() {
    latlongs = [];
    places = JSON.parse(sessionStorage.getItem('places'));
    console.log(places);
    places.forEach(function(i){
        i = i.split(",")
        latlongs.push({lat: parseFloat(i[0]), lng: parseFloat(i[1])});
    });
    console.log(latlongs);
    markers = [];
    var map = new google.maps.Map(document.getElementById('map'), {
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
          map: map
        }));
    });
    
    
//    console.log(markers[0].getPosition());
    var bounds = new google.maps.LatLngBounds();
    for (var i = 0; i < markers.length; i++) {
     bounds.extend(markers[i].getPosition());
    }

    map.fitBounds(bounds);
}


