import { Image, View } from "react-native";
import { Input } from "@/components/input";
import { Button } from "@/components/button";
import {
  FontAwesome6,
  MaterialIcons,
} from "@expo/vector-icons";
import { colors } from "@/styles/colors";
import { Link } from "expo-router";

export default function Register() {
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
          <Input.Field placeholder="Nome completo" />
        </Input>
        <Input>
          <MaterialIcons
            name="alternate-email"
            color={colors.green[200]}
            size={20}
          />
          <Input.Field placeholder="Email" keyboardType="email-address" />
        </Input>

        <Button title="Realizar inscrição" />

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
