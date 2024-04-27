document.addEventListener('DOMContentLoaded', async (event) => {

  const { invoke } = window.__TAURI__.tauri;
  window.all_activity = []

  window.travelMethod = "DRIVING"
  window.account = {}

  connected("", false)


  window.initMap = function() {
    window.googleMapInstance = new Google(); // Crée et stocke l'instance globalement
  };
  let overview = document.getElementById('overview')
  overview.addEventListener('click', () => {
    openModal()
  })

  document.addEventListener('keydown', function(event) {
    if (event.key === "Enter") {
      simulateClick('search')
    }
  });

  const login_button = document.getElementById("button_login")
  login_button.addEventListener('click', () => {
    openModal_login()
    const login = document.getElementById("login")
    const submit = document.getElementById("sub")
    const register = document.getElementById("register")

    divContainerLogin = document.getElementById("contentLogin")

    login.addEventListener('click', function() {
      document.getElementById('nom').style.display = 'none';
      document.getElementById('email').style.display = '';
      document.getElementById('username').style.display = 'none';
      document.getElementById('password').style.display = '';
    });

    register.addEventListener('click', function() {
      document.getElementById('nom').style.display = '';
      document.getElementById('email').style.display = '';
      document.getElementById('username').style.display = '';
      document.getElementById('password').style.display = '';
    });


    submit.addEventListener("click", () => {
      const username = document.getElementById('username').value
      const email = document.getElementById('email').value
      const name = document.getElementById('nom').value
      const password = document.getElementById("password").value

      if (!username && !email && !password && !name) {
        alert("vérifier vos informations, toutes les information sont requises")
        return;
      }

      const obj = {
        name: name,
        lastName: username,
        email: email,
        password: password
      }

      if (!username && !name) {
        invoke('login_api', { email: obj.email, password: obj.password }).then(() => {
          closeModal_login()
          alert("vous êtes connecté")
          connected(obj.email, true)
        }).catch(() => {
          console.log("une erreur est survenue lors de la connexion")
        })
      } else if (username && password && name && email) {
        invoke('create_account', obj).then(() => {
          closeModal_login()
          alert("vous êtes connecté")
          connected(obj.email, true)
        }).catch(() => {
          alert("erreur de création de compte vérifier les infos")
        })
      } else {
        alert("vérifier vos informations")
      }

      document.getElementById("button_disconnect").addEventListener('click', () => {
        alert("vous êtes deconnecté")
        connected("", false)
      });
    })
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
          case 'sport':
            commande = "get_enjoy"
          case 'bar':
            commande = "get_bar"
        }
        try {
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

              const addImage = document.createElement('p');
              // addImage.src = './assets/add.png'
              addImage.className = "addImage";
              addImage.textContent = '+'

              addImage.addEventListener('click', () => {
                /*if (addImage.textContent == "-") {
                  let index = window.all_activity.indexOf(restaurant);
                  if (index !== -1) {
                    window.all_activity.splice(index, 1);
                  }
                  addImage.textContent = "+"
                  alert("activité enlever du roadtrip")
                } else {
                  window.all_activity.push(restaurant)
                  alert('ativité ajouter avec succès')
                  addImage.textContent = '-'
                }*/
                toggleActivity(addImage, restaurant, window.all_activity)
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

function toggleActivity(element, restaurant, activityArray) {
  let index = activityArray.indexOf(restaurant);
  if (element.textContent === "-") {
    if (index !== -1) {
      activityArray.splice(index, 1);
    }
    element.textContent = "+";
    alert("Activité enlevée du roadtrip");
  } else {
    activityArray.push(restaurant);
    alert('Activité ajoutée avec succès');
    element.textContent = '-';
  }
  console.log(activityArray); // Debugging output
}

function contruction_modal() {

  document.getElementById('gauche').innerText = ''
  document.getElementById("droite").innerText = ''

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

function openModal_login() {
  document.getElementById('modal_login').style.display = "block";
  const closeButton = document.getElementById("close_login");
  closeButton.addEventListener('click', () => {
    closeModal_login();
  })
}


function closeModal_login() {
  document.getElementById('modal_login').style.display = "none";
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

    let divHead = document.createElement("div")
    divHead.className = 'divHead'

    let suppButton = document.createElement("span")
    suppButton.className = 'suppButton'
    suppButton.innerHTML = '&times;'

    let h2 = document.createElement('h2')
    h2.textContent = element.name

    let p = document.createElement('p')
    p.textContent = element.address

    suppButton.addEventListener('click', () => {
      let index = window.all_activity.indexOf(element);
      if (index !== -1) {
        window.all_activity.splice(index, 1);
      }
      contruction_modal()
    })

    divHead.appendChild(h2)
    divHead.appendChild(suppButton)
    divPrincipal.appendChild(divHead)
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


function simulateClick(divId) {
  var elem = document.getElementById(divId);
  if (elem) {
    elem.click();
  }
}

function connected(email, isConnected) {
  if (isConnected === true) {
    document.getElementById("button_login").style.display = 'none'
    document.getElementById("imgProfile").style.display = ''
    document.getElementById("emailProfile").style.display = ''
    document.getElementById("emailProfile").innerText = email
    document.getElementById("button_disconnect").style.display = ''
  } else {
    document.getElementById("emailProfile").innerText = ''
    document.getElementById("button_login").style.display = ''
    document.getElementById("imgProfile").style.display = 'none'
    document.getElementById("emailProfile").style.display = 'none'
    document.getElementById("button_disconnect").style.display = 'none'
  }
}
