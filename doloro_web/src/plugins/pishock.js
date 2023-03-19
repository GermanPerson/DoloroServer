export async function get_shocker_list() {
    return fetch(window.baseURL + "/api/pishock/list")
        .then(async response => {
            if (response.status !== 200) {
                let msg = await response.json();
                throw new Error(msg);
            }
            return response.json()
        }).then(data => {
            return data
        })
        .catch(error => {
            this.$toast.error(error)
            console.error(error)
        })
}

export async function create_shocker(sharecode) {
    return fetch(window.baseURL + "/api/pishock/create", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            sharecode: sharecode
        })
    })
        .then(response => response.json())
        .then(data => {
            return data
        })
        .catch(error => {
            console.error(error)
        })
}

export async function get_shocker_info(id) {
    return fetch(`${window.baseURL}/api/pishock/${id}`)
        .then(async response => {
            if (response.status !== 200) {
                let msg = await response.json();
                throw new Error(msg);
            }
            return response.json()
        })
        .then(data => {
            return data
        })
        .catch(error => {
            this.$toast.error(error)
            console.error(error)
        })
}

export async function execute_shock(shocker_id, intensity, duration, warn) {
    return fetch(`${window.baseURL}/api/pishock/${shocker_id}/shock`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "intensity": intensity,
            "duration": duration,
            "warn": warn
        })
    })
        .then(async response => {
            if (response.status === 200) {
                this.$toast.success("Shock sent successfully");
            } else {
                let msg = await response.json();
                throw new Error(msg);
            }
            return response.json()
        })
        .then(data => {
            return data
        })
        .catch(error => {
            this.$toast.error(error)
            console.error(error)
        })
}

export async function execute_vibrate(shocker_id, intensity, duration) {
    return fetch(`${window.baseURL}/api/pishock/${shocker_id}/vibrate`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "intensity": intensity,
            "duration": duration,
        })
    })
        .then(async response => {
            if (response.status === 200) {
                this.$toast.success("Vibrate sent successfully");
            } else {
                let msg = await response.json();
                throw new Error(msg);
            }
            return response.json()
        })
        .then(data => {
            return data
        })
        .catch(error => {
            this.$toast.error(error)
            console.error(error)
        })
}
