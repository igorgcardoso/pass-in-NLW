import { Alert, Image, View } from "react-native";
import { Input } from "@/components/input";
import { Button } from "@/components/button";
import { FontAwesome6, MaterialIcons } from "@expo/vector-icons";
import { colors } from "@/styles/colors";
import { Link, router } from "expo-router";
import { useState } from "react";
import { api } from "@/server/api";
import axios from "axios";
import { useBadgeStore } from "@/store/badge-store";

export default function Register() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [isLoading, setIsloading] = useState(false);

  const badgeStore = useBadgeStore();

  async function handleRegister() {
    if (!name.trim() || !email.trim()) {
      return Alert.alert("Inscrição", "Preencha todos os campos");
    }

    setIsloading(true);

    try {
      const event_id = "d90dbdf8-eeff-4137-b0f5-78b5b388f216";
      const response = await api.post(`/events/${event_id}/attendees`, {
        name,
        email,
      });

      if (response.data.attendeeId) {
        const badgeResponse = await api.get(
          `/attendees/${response.data.attendeeId}/badge`,
        );

        badgeStore.save(badgeResponse.data.badge);

        Alert.alert("Inscrição", "Inscrição realizada com sucesso", [
          {
            text: "Ok",
            onPress: () => router.push("/ticket"),
          },
        ]);
      }
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (
          String(error.response?.data.message).includes("already registered")
        ) {
          return Alert.alert("Inscrição", "Este e-mail já está inscrito");
        }
      }

      Alert.alert("Inscrição", "Não foi possível fazer a inscrição");
    } finally {
      setIsloading(false);
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
          <FontAwesome6
            name="user-circle"
            color={colors.green[200]}
            size={20}
          />
          <Input.Field placeholder="Nome completo" onChangeText={setName} />
        </Input>
        <Input>
          <MaterialIcons
            name="alternate-email"
            color={colors.green[200]}
            size={20}
          />
          <Input.Field
            placeholder="Email"
            keyboardType="email-address"
            onChangeText={setEmail}
          />
        </Input>

        <Button
          title="Realizar inscrição"
          onPress={handleRegister}
          isLoading={isLoading}
        />

        <Link
          href="/"
          className="mt-8 text-center font-bold text-base text-gray-100"
        >
          Já possui ingresso?
        </Link>
      </View>
    </View>
  );
}
