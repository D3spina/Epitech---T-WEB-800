document.addEventListener('DOMContentLoaded', async (event) => {

  const { invoke } = window.__TAURI__.tauri;
  window.all_activity = []

  window.travelMethod = "DRIVING"


  window.initMap = function() {
    window.googleMapInstance = new Google(); // Crée et stocke l'instance globalement
  };
  let overview = document.getElementById('overview')
  overview.addEventListener('click', () => {
    openModal()

  })



  document.getElementById('search').addEventListener('click', function() {
    const departValue = document.getElementById('depart').value;
    const arriveValue = document.getElementById('arrivee').value;
    const rayonValue = document.getElementById('rayon').value;

    if (departValue == "" || arriveValue == "" || rayonValue == "") {
      alert("veuillez renseigner les champ de saisis")
      return;
    }

    let radios = document.querySelectorAll('input[type="radio"][name="activity"]');
    radios.forEach(function(radio) {
      radio.addEventListener('change', async function() {
        let div = document.getElementById('data-list');
        div.innerHTML = '';
        window.googleMapInstance.removeOtherMarkers()
        console.log('L\'activité sélectionnée est : ' + this.value);
        switch (this.value) {
          case 'restaurant':
            commande = "get_restaurants"
            break;
          case 'accommodation':
            commande = "get_sleep"
            break;

          //case 'sport':
          //  commande = "get_sport"
          //case 'bar':
          //  commande = "get_bar"
        }
        // console.log(arriveValue)
        try {
          // Premièrement, attendez que les restaurants du départ et de l'arrivée soient chargés.
          Promise.all([
            //loadRestaurants(invoke, departValue, parseInt(rayonValue), commande),
            // loadRestaurants(invoke, arriveValue, parseInt(rayonValue), commande)
          ]);

          // Maintenant, si vous avez des villes, chargez les restaurants pour chaque ville.
          if (window.googleMapInstance && window.googleMapInstance.cities) {
            window.googleMapInstance.cities.delete(departValue);
            // window.googleMapInstance.delete(arriveValue)
            let citiesArray = Array.from(window.googleMapInstance.cities);
            Promise.all(citiesArray.map(city => {
              console.log(city); // Ou chargez des restaurants pour cette ville
              loadRestaurants(invoke, city, parseInt(rayonValue), commande);
            }));
          }

        } catch (error) {
          console.error('Error loading restaurants:', error);
        }
      });
    });


    if (!departValue || !arriveValue) {
      alert("Veuillez vérifier vos villes");
      return;
    } else {
      setupRoutes(departValue, arriveValue, 'DRIVING').then((response) => {
        travelInfos(response.totalDistance, response.totalTime)
        // console.log(response)
      }).catch(error => {
        console.log(error)
      })
      document.getElementById('mode_transport').addEventListener('click', async function(event) {
        if (event.target.tagName === 'IMG') {
          const modeDiv = event.target.parentNode;
          const mode = modeDiv.id;
          if (mode) {
            console.log('Mode de transport sélectionné:', mode.toUpperCase());
            setupRoutes(departValue, arriveValue, mode.toUpperCase()).then((response) => {
              travelInfos(response.totalDistance, response.totalTime)
            }).catch(err => {
              console.log(err)
            })
          }
        }
      });

    }
  });


  window.onclick = function(event) {
    var modal = document.getElementById('myModal');
    if (event.target == modal) {
      modal.style.display = "none";
    }
  }
});


async function loadRestaurants(invoke, ville, ratio, commande) {
  try {
    invoke(commande, { ville: ville, radius: ratio })
      .then((response) => {
        const restaurants = response;
        const dataList = document.getElementById('data-list');
        // dataList.innerHTML = '';
        //setTimeout("patienté", 2000)

        let count = 0
        if (restaurants && Array.isArray(restaurants)) {
          restaurants.forEach(restaurant => {
            if (restaurant.rating > 4) {
              count += 1;


              window.googleMapInstance.geocodeAddress(restaurant)

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
              rat = parseFloat(restaurant.rating.toFixed(1));
              if (rat == 0) {
                rating.textContent = "pas noté"
              } else {
                rating.textContent = rat;
              }
              rating.className = "rating";

              const address = document.createElement('p');
              address.textContent = restaurant.address;
              address.className = 'address'

              const addImage = document.createElement('img');
              addImage.src = './assets/add.png'
              addImage.className = "addImage";

              addImage.addEventListener('click', () => {
                window.all_activity.push(restaurant)
                alert('ativité ajouter avec succès')
              })

              if (restaurants.picture != "") {
                imageContainer.appendChild(image)
              }


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

            }
          });
        }
      })

  } catch (error) {
    console.error('Failed to load restaurants', error);
  }
}

function contruction_modal() {
  all_activity = window.all_activity
  const depart = document.getElementById('depart').value
  const arrive = document.getElementById('arrivee').value

  const departModal = document.getElementById('depart-modal')
  const arriveModal = document.getElementById('arrivee-modal')

  departModal.textContent = depart
  arriveModal.textContent = arrive

  let [first, second] = split_liste(all_activity)

  add_div(first, 'gauche')
  add_div(second, 'droite')

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

async function setupRoutes(villeDepart, villeArrive, methode) {
  try {
    if (window.googleMapInstance) {
      return await window.googleMapInstance.travelRoute(villeDepart, villeArrive, methode);
    } else {
      setTimeout(() => setupRoutes(villeDepart, villeArrive, methode), 100);
    }
  } catch (error) {
    console.error('Erreur lors de l\'obtention des informations d\'itinéraire:', error);
  }
}


function openModal() {
  document.getElementById('myModal').style.display = "block";
  const closeButton = document.getElementById("close");
  const printButton = document.getElementById("print");
  closeButton.addEventListener('click', () => {
    closeModal();
  })
  printButton.addEventListener("click", () => {
    imprimerPage();
  })

  contruction_modal()
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

function split_liste(all_activity) {
  let totalLength = all_activity.length;
  let sizeFirstList = Math.ceil(totalLength / 2);
  let firstList = all_activity.slice(0, sizeFirstList);
  let secondList = all_activity.slice(sizeFirstList);
  return [firstList, secondList];
}

function add_div(liste, cote) {

  divContainer = document.getElementById(cote);

  liste.forEach(element => {
    let divPrincipal = document.createElement('div')

    let h2 = document.createElement('h2')
    h2.textContent = element.name

    let p = document.createElement('p')
    p.textContent = element.address

    divPrincipal.appendChild(h2)
    divPrincipal.appendChild(p)
    divContainer.appendChild(divPrincipal)
  });
}


function travelInfos(distance, temps) {
  let hours = Math.floor(temps / 60);
  let minutes = temps % 60

  document.getElementById('distance').innerHTML = `${parseInt(distance)} Km`
  document.getElementById('temps').innerHTML = `${hours}H ${parseInt(minutes)} min`

}
