import { colors } from "@/styles/colors";
import { ReactNode } from "react";
import { TextInput, View, TextInputProps } from "react-native";

function Input({ children }: { children: ReactNode }) {
  return (
    <View className="h-14 w-full flex-row items-center gap-3 rounded-lg border border-green-400 p-3">
      {children}
    </View>
  );
}

function Field({ ...props }: TextInputProps) {
  return (
    <TextInput
      placeholderTextColor={colors.gray[200]}
      className="flex-1 font-regular text-base text-white"
      {...props}
    />
  );
}

Input.Field = Field;

export { Input };
