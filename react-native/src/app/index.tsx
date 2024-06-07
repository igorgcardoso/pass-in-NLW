import { Alert, Image, View } from "react-native";
import { Input } from "@/components/input";
import { Button } from "@/components/button";
import { MaterialCommunityIcons } from "@expo/vector-icons";
import { colors } from "@/styles/colors";
import { Link } from "expo-router";
import { useState } from "react";

export default function Home() {
  const [code, setCode] = useState("");

  function handleAccessCredential() {
    if (!code.trim()) {
      return Alert.alert("Ingresso", "Informe o código do ingresso!");
    }
  }

  return (
    <View className="flex-1 items-center justify-center bg-green-500 p-8">
      <Image
        source={require("@/assets/logo.png")}
        className="h-16"
        resizeMode="contain"
      />
      <View className="mt-12 w-full gap-3">
        <Input>
          <MaterialCommunityIcons
            name="ticket-confirmation-outline"
            color={colors.green[200]}
            size={20}
          />
          <Input.Field
            placeholder="Código do ingresso"
            onChangeText={setCode}
          />
        </Input>

        <Button title="Acessar credencial" onPress={handleAccessCredential} />

        <Link
          href="/register"
          className="mt-8 text-center font-bold text-base text-gray-100"
        >
          Ainda não possui ingresso?
        </Link>
      </View>
    </View>
  );
}
