import axios from 'axios';
import authHeader from './auth-header';
import { apiUrl } from "./api_url";

// if (typeof window !== "undefined") {
// const API_URL = "http://" + window.location.hostname + ":3000/";

//  }

class UserService {
  getHighscore() {
    return axios.get(apiUrl() + 'highscore', { headers: authHeader() });
  }
  getMeters() {
    return axios.get(apiUrl() + 'get_meters', { headers: authHeader() });
  }
  createEntry(meters) {
    return axios.post(apiUrl() + 'create_entry', { meters: meters }, { headers: authHeader() });
  }
}
export default new UserService();
