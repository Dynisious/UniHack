$(document).ready(function() {
    a = sessionStorage.getItem('places')
    console.log(a);
});

function initMap() {
    var markers = a;
    var latlng = new google.maps.LatLng(39.305, -76.617);
    map = new google.maps.Map(document.getElementById('map'), {
      center: latlng,
      zoom: 12
    });
    var marker = new google.maps.Marker({
      position: latlng,
      map: map
    });
    var marker2 = new google.maps.Marker({
      position: latlng2,
      map: map
    });
    var bounds = new google.maps.LatLngBounds();
    for (var i = 0; i < markers.length; i++) {
     bounds.extend(markers[i].getPosition());
    }

    map.fitBounds(bounds);
}


