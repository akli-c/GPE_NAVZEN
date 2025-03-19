import React, { useState } from "react";
import {
  View,
  TextInput,
  Text,
  StyleSheet,
  Alert,
  TouchableOpacity,
  ImageBackground,
} from "react-native";
import DropDownPicker from "react-native-dropdown-picker";
import apiService from "../services/ApiService";
import { Ionicons } from "@expo/vector-icons";

const RegisterScreen = ({ navigation }) => {
  const [email, setEmail] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [level, setLevel] = useState(null);
  const [open, setOpen] = useState(false);
  const [error, setError] = useState("");

  const handleRegister = async () => {
    if (!email || !username || !password) {
      setError("⚠️ Tous les champs doivent être remplis.");
      return;
    }

    try {
      const response = await apiService.register(
        email,
        username,
        password,
        level
      );

      if (response) {
        Alert.alert("🎉 Succès", "Compte créé avec succès !");
        navigation.navigate("LoginScreen");
      }
    } catch (error) {
      setError(
        error.message || "❗ Une erreur est survenue lors de l'inscription."
      );
      console.error("Erreur:", error);
    }
  };

  return (
    <ImageBackground
      source={require("../assets/register_background.jpg")}
      style={styles.background}
    >
      <View style={styles.overlay} />
      <Text style={styles.appTitle}>🌲 Navzen-Co</Text>

      <View style={styles.container}>
        <Text style={styles.title}>📝 Inscription</Text>

        <View style={styles.inputContainer}>
          <Ionicons
            name="mail-outline"
            size={20}
            color="#aaa"
            style={styles.icon}
          />
          <TextInput
            style={styles.input}
            placeholder="Email"
            value={email}
            onChangeText={setEmail}
            keyboardType="email-address"
            autoCapitalize="none"
            placeholderTextColor="#aaa"
          />
        </View>

        <View style={styles.inputContainer}>
          <Ionicons
            name="person-outline"
            size={20}
            color="#aaa"
            style={styles.icon}
          />
          <TextInput
            style={styles.input}
            placeholder="Nom d'utilisateur"
            value={username}
            onChangeText={setUsername}
            autoCapitalize="none"
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
            onChangeText={setPassword}
            secureTextEntry
            autoCapitalize="none"
            placeholderTextColor="#aaa"
          />
        </View>

        <DropDownPicker
          open={open}
          value={level}
          items={[
            { label: "🥾 Débutant", value: "Débutant" },
            { label: "⛰️ Intermédiaire", value: "Intermédiaire" },
            { label: "🏔️ Expert", value: "Expert" },
          ]}
          setOpen={setOpen}
          setValue={setLevel}
          placeholder="🎯 Sélectionnez votre niveau"
          containerStyle={styles.dropdownContainer}
          style={styles.dropdown}
          placeholderStyle={{ color: "#aaa" }}
          textStyle={{ textAlign: "center" }}
          dropDownContainerStyle={styles.dropdownItem}
        />

        {error ? <Text style={styles.error}>{error}</Text> : null}

        <TouchableOpacity style={styles.button} onPress={handleRegister}>
          <Text style={styles.buttonText}>✅ S'inscrire</Text>
        </TouchableOpacity>

        <TouchableOpacity onPress={() => navigation.navigate("LoginScreen")}>
          <Text style={styles.loginText}>
            🔑 Déjà un compte ? Connectez-vous
          </Text>
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
  title: {
    fontSize: 28,
    fontWeight: "bold",
    color: "#2c3e50",
    textAlign: "center",
    marginBottom: 20,
  },
  appTitle: {
    fontSize: 32,
    fontWeight: "bold",
    color: "#2ecc71",
    textAlign: "center",
    paddingBottom: 40,
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
  dropdownContainer: {
    height: 40,
    marginBottom: 20,
  },
  dropdown: {
    backgroundColor: "#fff",
    borderColor: "#ccc",
  },
  dropdownItem: {
    borderWidth: 0,
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
  loginText: {
    color: "#2c3e50",
    textAlign: "center",
    marginTop: 15,
    fontWeight: "500",
    textDecorationLine: "underline",
  },
  error: {
    color: "#e74c3c",
    marginBottom: 10,
    textAlign: "center",
    fontWeight: "bold",
  },
});

export default RegisterScreen;
