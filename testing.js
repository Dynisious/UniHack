var chosen=[];
var listOfImages=["beer.jpg","shopping.jpeg","sightseeing.jpeg","movie.jpg","basket.jpg","gambling.jpeg","fish.jpg","footy.jpeg","library.jpeg","tea.jpeg","nature.jpeg","rock.jpeg","hiking.jpeg","soccer.jpg","dating.jpeg","busking.jpg","chess.jpeg","dancing.jpg","kangaroo.jpg","gaming.jpg"]
function generate(){

	
	var show = 4;
	for(i=0;i<show;i++){
		var x = document.createElement("IMG");
		x.setAttribute("id",i)
		x.setAttribute("class","pic")
		x.setAttribute("onclick","Choose(this)")
    x.setAttribute("src", listOfImages[Math.floor(Math.random() * 11)]);
    document.body.appendChild(x);
	}
	function randomWholeNum() {

  // Only change code below this line.

  	return Math.random();
	}
}
function remove(){
	for(i=0;i<4;i++){
		var elem = document.getElementById(i.toString())
		elem.parentNode.removeChild(elem);

	}
	console.log("removelastpage")
}
function Choose(image){
	
	var pathElements = image.src.replace(/\/$/, '').split('/');
	var lastFolder = pathElements[pathElements.length - 1];
	chosen.push(lastFolder);
	console.log(listOfImages.indexOf(lastFolder))
	console.log(chosen);
}
$(document).ready(function() {
    var cart = JSON.parse(window.localStorage.getItem("images_selected"))
    console.log(cart);
    //
    var vPool="";
    jQuery.each(cart, function(i, val) {
        vPool += val + "<br>";
    });

    //We add vPool HTML content to #myDIV
    $('#myDIV').html(vPool);
});

/*function initMap() {
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
}*/
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


