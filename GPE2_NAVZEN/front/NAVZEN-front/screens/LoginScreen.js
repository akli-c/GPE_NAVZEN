import React, { useEffect, useState } from "react";
import {
  View,
  TextInput,
  Text,
  StyleSheet,
  ImageBackground,
  TouchableOpacity,
  Alert,
} from "react-native";
import AsyncStorage from "@react-native-async-storage/async-storage";
import apiService from "../services/ApiService";
import { Ionicons } from "@expo/vector-icons";

const LoginScreen = ({ navigation }) => {
  const [login, setLogin] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  useEffect(() => {
    const checkToken = async () => {
      const token = await AsyncStorage.getItem("jwtToken");
      if (token) {
        try {
          navigation.replace("Location");
        } catch (err) {
          await AsyncStorage.removeItem("jwtToken");
        }
      }
    };
    checkToken();
  }, []);

  const handleLogin = async () => {
    try {
      await apiService.login(login, password);
      navigation.replace("Location");
    } catch (error) {
      setError(
        error.message || "Échec de la connexion. Vérifiez vos identifiants."
      );
    }
  };

  return (
    <ImageBackground
      source={require("../assets/login_background.webp")}
      style={styles.background}
    >
      <View style={styles.overlay} />
      <Text style={styles.appTitle}>🌲 Navzen-Co</Text>
      <View style={styles.container}>
        <Text style={styles.title}>🔐 Connexion</Text>

        <View style={styles.inputContainer}>
          <Ionicons
            name="person-outline"
            size={20}
            color="#aaa"
            style={styles.icon}
          />
          <TextInput
            style={styles.input}
            placeholder="Identifiant (Email ou Nom d'utilisateur)"
            value={login}
            autoCapitalize="none"
            onChangeText={setLogin}
            keyboardType="email-address"
            placeholderTextColor="#aaa"
          />
        </View>

        <View style={styles.inputContainer}>
          <Ionicons
            name="lock-closed-outline"
            size={20}
            color="#aaa"
            style={styles.icon}
          />
          <TextInput
            style={styles.input}
            placeholder="Mot de passe"
            value={password}
            autoCapitalize="none"
            onChangeText={setPassword}
            secureTextEntry
            placeholderTextColor="#aaa"
          />
        </View>

        {error ? <Text style={styles.error}>❗{error}</Text> : null}

        <TouchableOpacity style={styles.button} onPress={handleLogin}>
          <Text style={styles.buttonText}>🚀 Se connecter</Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={styles.registerButton}
          onPress={() => navigation.replace("RegisterScreen")}
        >
          <Text style={styles.registerButtonText}>📝 Créer un compte</Text>
        </TouchableOpacity>
      </View>
    </ImageBackground>
  );
};

const styles = StyleSheet.create({
  background: {
    flex: 1,
    justifyContent: "center",
    resizeMode: "cover",
  },
  overlay: {
    position: "absolute",
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    backgroundColor: "rgba(0, 0, 0, 0.6)",
  },
  container: {
    padding: 20,
    backgroundColor: "rgba(255, 255, 255, 0.9)",
    borderRadius: 15,
    marginHorizontal: 20,
    shadowColor: "#000",
    shadowOffset: { width: 0, height: 5 },
    shadowOpacity: 0.2,
    shadowRadius: 10,
    elevation: 8,
  },
  appTitle: {
    fontSize: 32,
    fontWeight: "bold",
    color: "#2ecc71",
    textAlign: "center",
    paddingBottom: 40,
  },
  title: {
    fontSize: 24,
    fontWeight: "bold",
    color: "#2c3e50",
    textAlign: "center",
    marginBottom: 20,
  },
  inputContainer: {
    flexDirection: "row",
    alignItems: "center",
    backgroundColor: "#f1f1f1",
    borderRadius: 8,
    marginBottom: 15,
    paddingHorizontal: 10,
  },
  icon: {
    marginRight: 8,
  },
  input: {
    flex: 1,
    height: 45,
    color: "#333",
    fontSize: 15,
  },
  button: {
    backgroundColor: "#00a86b",
    paddingVertical: 14,
    borderRadius: 10,
    marginTop: 10,
  },
  buttonText: {
    color: "#fff",
    textAlign: "center",
    fontWeight: "bold",
    fontSize: 16,
  },
  error: {
    color: "#e74c3c",
    marginBottom: 10,
    textAlign: "center",
    fontWeight: "bold",
  },
  registerButton: {
    marginTop: 15,
    paddingVertical: 12,
    borderRadius: 10,
    backgroundColor: "#3498db",
  },
  registerButtonText: {
    color: "#fff",
    textAlign: "center",
    fontWeight: "bold",
    fontSize: 16,
  },
});

export default LoginScreen;
