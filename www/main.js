loadList()

function loadList() {
    try {       
        // get the current list from the backend
        fetch(document.location.origin + "/api/getlist")
            .then(res => {
                const html_list = document.getElementById("list");
                res.json().then(json => {
                    for (const i in json) {
                        let date = new Date(json[i].timestamp.secs*1000);
                        const item = 
                            `<div class="list-item">
                                <p> ${json[i].name} </p>
                                <br>
                                <p class="vote"> ${date} <p>
                            </div>`
                        // add
                        html_list.insertAdjacentHTML("beforeend", item);
                    }
                });
            })
        } catch (e) {
        console.log("api fail" + e);
    }
}

