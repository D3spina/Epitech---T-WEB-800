document.addEventListener('DOMContentLoaded', async (event) => {

  const { invoke } = window.__TAURI__.tauri;

  window.initMap = function() {
    window.googleMapInstance = new Google(); // Crée et stocke l'instance globalement
  };



  document.getElementById('search').addEventListener('click', async function() {
    const departValue = document.getElementById('depart').value;
    const arriveValue = document.getElementById('arrivee').value;
    const rayonValue = document.getElementById('rayon').value;

    let radios = document.querySelectorAll('input[type="radio"][name="activity"]');
    radios.forEach(function(radio) {
      radio.addEventListener('change', function() {
        let div = document.getElementById('data-list');
        div.innerHTML = '';
        console.log('L\'activité sélectionnée est : ' + this.value);
        switch (this.value) {
          case 'restaurant':
            window.googleMapInstance.geocodeAddress()
            loadRestaurants(invoke, departValue, parseInt(rayonValue))
            loadRestaurants(invoke, arriveValue, parseInt(rayonValue))
        }
      });
    });


    if (!departValue || !arriveValue) {
      alert("Veuillez vérifier vos villes");
      return;
    } else {
      setupRoutes(departValue, arriveValue);

    }
  });


  window.onclick = function(event) {
    var modal = document.getElementById('myModal');
    if (event.target == modal) {
      modal.style.display = "none";
    }
  }
});


async function loadRestaurants(invoke, ville, ratio) {
  try {
    invoke('get_restaurants', { ville: ville, ratio: ratio })
      .then((response) => {
        const restaurants = response;
        const dataList = document.getElementById('data-list');
        // dataList.innerHTML = '';

        let count = 0
        if (restaurants && Array.isArray(restaurants)) {
          restaurants.forEach(restaurant => {
            count += 1;
            const marker = window.googleMapInstance.geocodeAddress(restaurant.address);
            marker.addEventListener('click', function() {
              scrollToRestaurant(this.restaurantId);
            });
            const restaurantDiv = document.createElement('div');
            restaurantDiv.className = 'restaurant';

            const infoDiv = document.createElement('div');

            const infoDivRatingName = document.createElement('div')

            const imageContainer = document.createElement('div');

            const name = document.createElement('h3');
            name.textContent = restaurant.name;
            name.className = 'name';

            const image = document.createElement('img');
            image.src = restaurant.picture;

            const rating = document.createElement('p');
            rating.textContent = parseFloat(restaurant.rating.toFixed(1));
            rating.className = "rating";

            const address = document.createElement('p');
            address.textContent = restaurant.address;
            address.className = 'address'

            const addImage = document.createElement('img');
            addImage.src = './assets/add.png'
            addImage.className = "addImage";


            imageContainer.appendChild(image)


            infoDivRatingName.appendChild(name);
            infoDivRatingName.appendChild(rating);
            infoDivRatingName.appendChild(imageContainer);

            infoDivRatingName.className = "topContainer";
            infoDiv.appendChild(address);
            infoDiv.appendChild(addImage);
            restaurantDiv.appendChild(infoDivRatingName)
            restaurantDiv.appendChild(infoDiv);
            restaurantDiv.id = `${ville} - ${count}`
            dataList.appendChild(restaurantDiv);
          });
        }
      })

  } catch (error) {
    console.error('Failed to load restaurants', error);
  }
}



async function getLoc(invoke, ville) {
  try {
    const response = await invoke('get_localisation', { ville: ville });
    return response;
  } catch (err) {
    console.error('Failed to fetch location:', err);
    throw err;
  }
}

function setupRoutes(villeDepart, villeArrive) {
  if (window.googleMapInstance) {
    window.googleMapInstance.travelRoute(villeDepart, villeArrive);
  } else {
    setTimeout(setupRoutes, 100);
  }
}


function openModal() {
  document.getElementById('myModal').style.display = "block";
}

function closeModal() {
  document.getElementById('myModal').style.display = "none";
}

function imprimerPage() {
  window.print();
}

function chargementCarrousel() {
  let checkbox = document.getElementsByName('activity');
  checkbox.addEventListener('click', () => {
    console.log(checkbox.value)
  })
}

function scrollToRestaurant(restaurantId) {
  var element = document.getElementById(restaurantId);
  if (element) {
    element.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }
}

