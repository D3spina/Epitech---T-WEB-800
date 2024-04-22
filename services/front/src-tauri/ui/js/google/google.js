class Google {



  constructor() {
    try {
      this.map = this.initMap();
      this.directionsService = new google.maps.DirectionsService();
      this.directionsRenderer = new google.maps.DirectionsRenderer();
      this.directionsRenderer.setMap(this.map);
      this.citySet = new Set();
      this.allMarker = []
      this.cities = new Set();
      this.geocoder = new google.maps.Geocoder();
    } catch (error) {
      console.error('Failed to initialize Google Maps:', error);
    }
  }

  initMap() {
    try {
      let map = new google.maps.Map(document.getElementById('map'), {
        center: { lat: 48.68, lng: 6.2 },
        zoom: 10
      });
      console.log('Map initialized.');
      return map;
    } catch (error) {
      console.error('Map initialization failed:', error);
      return null;
    }
  }

  addMarker(lat, lng) {
    var location = { lat: lat, lng: lng };
    var marker = new google.maps.Marker({
      position: location,
      map: this.map,
    });
    marker.addListener('click', () => {
      alert('Marqueur cliqué');
    });
  }

  geocodeAddress(obj) {
    // console.log(obj)
    return new Promise((resolve, reject) => {
      if (!obj.address) {
        reject("Address is empty");
        return;
      }
      this.geocoder.geocode({ 'address': obj.address }, (results, status) => {
        if (status === 'OK') {
          this.map.setCenter(results[0].geometry.location);
          const marker = new google.maps.Marker({
            map: this.map,
            position: results[0].geometry.location,
            title: obj.name,
          });
          var infoWindow = new google.maps.InfoWindow({
            content: `<h3 class="title_card">nom: ${obj.name}</h3><p class="rating">note sur 5: ${parseFloat(obj.rating.toFixed(1))}</p>`
          });

          marker.addListener('click', function() {
            infoWindow.open(this.map, marker);
          });

          this.allMarker.push(marker)
          resolve(marker);
        } else {
          console.error('Failed to geocode address:', obj.address, 'Status:', status);
          reject('Geocode was not successful: ' + status);
        }
      });
    });
  }

  removeOtherMarkers() {
    this.allMarker.forEach(marker => {
      marker.setMap(null);
    })
  }


  // bicycling, driving, walking, transit, motorcycle
  travelRoute(start, end, travelMode) {
    this.directionsService.route({
      origin: start,
      destination: end,
      travelMode: travelMode
    }, (response, status) => {
      if (status === 'OK') {
        this.directionsRenderer.setDirections(response);
        this.extractCitiesFromRoute(response);
      } else {
        console.error('Erreur dans la demande d\'itinéraire: ' + status);
      }
    });
  }

  extractCitiesFromRoute(directionResult) {
    let legs = directionResult.routes[0].legs;
    legs.forEach(leg => {
      this.getCityFromCoords(leg.start_location);
      leg.steps.forEach(step => {
        this.getCityFromCoords(step.start_location);
      });
    });
  }

  getCityFromCoords(location) {
    this.geocoder.geocode({ 'location': location }, (results, status) => {
      if (status === 'OK' && results[0]) {
        let addressComponents = results[0].address_components;
        let city = addressComponents.find(component => component.types.includes('locality'));
        if (city) {
          if (!this.cities.has(city.long_name)) {
            this.cities.add(city.long_name);
            // console.log('City:', city.long_name);
          }
        }
      } else {
        console.error('Geocoder failed due to: ' + status);
      }
    });
  }


}


