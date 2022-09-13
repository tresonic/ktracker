import axios from "axios";
import { apiUrl } from "./api_url";
// if (typeof window !== "undefined") {
// const API_URL = "http://" + window.location.hostname + ":3000/";

// }

class AuthService {
  login(username, pass) {
    console.log(apiUrl());
    return axios
      .post(apiUrl() + "authorize", {
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
    return axios.post(apiUrl() + "create_user", {
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

  isLoggedIn() {
    return this.getCurrentUser() != null;
  }
}

export default new AuthService();
