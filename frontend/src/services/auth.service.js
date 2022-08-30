import axios from "axios";
// if (typeof window !== "undefined") {
// const API_URL = "http://" + window.location.hostname + ":3000/";
const API_URL = "/api/";
// }

class AuthService {
  login(username, pass) {
    return axios
      .post(API_URL + "authorize", {
        username,
        pass
      })
      .then(response => {
        if (response.data.access_token && (typeof window !== "undefined")) {
          localStorage.setItem("user", JSON.stringify(response.data));
        }
        return response.data;
      }).catch(error => {
        console.log(error);
        return null;
      });
  }

  logout() {
    if (typeof window !== "undefined") {
      localStorage.removeItem("user");
    } else {
      return null;
    }
  }

  register(username, email, pass) {
    return axios.post(API_URL + "create_user", {
      username,
      pass,
      email
    }).then(response => {
      return response.data;
    }).catch(error => {
      console.log(error);
      return null;
    });
  }

  getCurrentUser() {
    if (typeof window !== "undefined") {
      return JSON.parse(localStorage.getItem('user'));
    } else {
      return null;
    }
  }
}

export default new AuthService();
