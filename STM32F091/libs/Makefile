# Project: VirtualMeter

CROSS	 = arm-none-eabi-
CPP      = $(CROSS)g++
CC       = $(CROSS)gcc
STRIP    = $(CROSS)strip
RM       = busybox rm -f

OBJ      = obj/eeprom_1.o obj/eeprom_2.o obj/rtc.o obj/rs485_1.o obj/rs485_2.o obj/vuart3.o obj/vuart4.o obj/vuart1.o obj/vuart2.o obj/battery.o obj/leds.o obj/buzzer.o obj/relay.o obj/lcd.o obj/cpu.o obj/delay.o obj/jiffy.o obj/power.o obj/trace.o obj/axdr.o obj/bcd.o obj/button.o obj/magnetic.o obj/keys.o obj/flash.o obj/eeprom.o obj/meter.o  obj/crc.o obj/stm32f0xx_adc.o obj/stm32f0xx_can.o obj/stm32f0xx_cec.o obj/stm32f0xx_comp.o obj/stm32f0xx_crc.o obj/stm32f0xx_crs.o obj/stm32f0xx_dac.o obj/stm32f0xx_dbgmcu.o obj/stm32f0xx_dma.o obj/stm32f0xx_exti.o obj/stm32f0xx_flash.o obj/stm32f0xx_gpio.o obj/stm32f0xx_i2c.o obj/stm32f0xx_iwdg.o obj/stm32f0xx_misc.o obj/stm32f0xx_pwr.o obj/stm32f0xx_rcc.o obj/stm32f0xx_rtc.o obj/stm32f0xx_spi.o obj/stm32f0xx_syscfg.o obj/stm32f0xx_tim.o obj/stm32f0xx_usart.o obj/stm32f0xx_wwdg.o obj/stm32f0xx_it.o

LINKOBJ  = obj/eeprom_1.o obj/eeprom_2.o obj/rtc.o obj/rs485_1.o obj/rs485_2.o obj/vuart3.o obj/vuart4.o obj/vuart1.o obj/vuart2.o obj/battery.o obj/leds.o obj/buzzer.o obj/relay.o obj/lcd.o obj/cpu.o obj/delay.o obj/jiffy.o obj/power.o obj/trace.o obj/axdr.o obj/bcd.o obj/button.o obj/magnetic.o obj/keys.o obj/flash.o obj/eeprom.o obj/meter.o  obj/crc.o obj/stm32f0xx_adc.o obj/stm32f0xx_can.o obj/stm32f0xx_cec.o obj/stm32f0xx_comp.o obj/stm32f0xx_crc.o obj/stm32f0xx_crs.o obj/stm32f0xx_dac.o obj/stm32f0xx_dbgmcu.o obj/stm32f0xx_dma.o obj/stm32f0xx_exti.o obj/stm32f0xx_flash.o obj/stm32f0xx_gpio.o obj/stm32f0xx_i2c.o obj/stm32f0xx_iwdg.o obj/stm32f0xx_misc.o obj/stm32f0xx_pwr.o obj/stm32f0xx_rcc.o obj/stm32f0xx_rtc.o obj/stm32f0xx_spi.o obj/stm32f0xx_syscfg.o obj/stm32f0xx_tim.o obj/stm32f0xx_usart.o obj/stm32f0xx_wwdg.o obj/stm32f0xx_it.o

INCS     = -I"Libraries/Check/Inc" -I"Libraries/Mbed/tls" -I"Libraries/Convert/Inc" -I"Libraries/trace/Inc" -I"Devices/common/Inc" -I"Devices/battery/Inc" -I"Devices/basic/Inc" -I"Devices/buses/Inc" -I"Devices/leds/Inc" -I"Devices/eeprom/Inc" -I"Devices/buzzer/Inc" -I"Devices/keys/Inc" -I"Devices/lcd/Inc" -I"Devices/rtc/Inc" -I"Devices/sensor/Inc" -I"Devices/serial/Inc" -I"Devices/metering/Inc" -I"Devices/relay/Inc" -I"Devices/flash/Inc" -I"Devices/basic/Inc/STM32F0xx/STM32F0xx_StdPeriph_Lib"  -I"Devices/basic/Inc/STM32F0xx/cmsis"  -I"Devices\basic\Inc\STM32F0xx\Interrupts" -I"Kernel/Inc"

BIN      = libdevices.a
LIBS     =
CFLAGS   = $(INCS) -O3 -std=c99 -march=armv6-m -DSTM32F091 -DUSE_STDPERIPH_DRIVER -DDEMO_STM32F091

.PHONY: all all-before all-after clean clean-custom

all: all-before $(BIN) all-after

clean: clean-custom
	${RM} $(OBJ) $(BIN)

$(BIN): $(OBJ)
	$(CROSS)ar r $(BIN) $(LINKOBJ)
	$(CROSS)ranlib $(BIN)

obj/eeprom_1.o: Devices/eeprom/Src/eeprom_1.c
	$(CC) -c Devices/eeprom/Src/eeprom_1.c -o obj/eeprom_1.o $(CFLAGS)

obj/eeprom_2.o: Devices/eeprom/Src/eeprom_2.c
	$(CC) -c Devices/eeprom/Src/eeprom_2.c -o obj/eeprom_2.o $(CFLAGS)

obj/rtc.o: Devices/rtc/Src/rtc.c
	$(CC) -c Devices/rtc/Src/rtc.c -o obj/rtc.o $(CFLAGS)

obj/rs485_1.o: Devices/serial/Src/rs485_1.c
	$(CC) -c Devices/serial/Src/rs485_1.c -o obj/rs485_1.o $(CFLAGS)

obj/rs485_2.o: Devices/serial/Src/rs485_2.c
	$(CC) -c Devices/serial/Src/rs485_2.c -o obj/rs485_2.o $(CFLAGS)

obj/battery.o: Devices/battery/Src/battery.c
	$(CC) -c Devices/battery/Src/battery.c -o obj/battery.o $(CFLAGS)

obj/leds.o: Devices/leds/Src/leds.c
	$(CC) -c Devices/leds/Src/leds.c -o obj/leds.o $(CFLAGS)

obj/buzzer.o: Devices/buzzer/Src/buzzer.c
	$(CC) -c Devices/buzzer/Src/buzzer.c -o obj/buzzer.o $(CFLAGS)

obj/relay.o: Devices/relay/Src/relay.c
	$(CC) -c Devices/relay/Src/relay.c -o obj/relay.o $(CFLAGS)

obj/lcd.o: Devices/lcd/Src/lcd.c
	$(CC) -c Devices/lcd/Src/lcd.c -o obj/lcd.o $(CFLAGS)

obj/disk.o: Kernel/Src/disk.c
	$(CC) -c Kernel/Src/disk.c -o obj/disk.o $(CFLAGS)

obj/cpu.o: Devices/basic/Src/cpu.c
	$(CC) -c Devices/basic/Src/cpu.c -o obj/cpu.o $(CFLAGS)

obj/delay.o: Devices/basic/Src/delay.c
	$(CC) -c Devices/basic/Src/delay.c -o obj/delay.o $(CFLAGS)

obj/jiffy.o: Devices/basic/Src/jiffy.c
	$(CC) -c Devices/basic/Src/jiffy.c -o obj/jiffy.o $(CFLAGS)

obj/power.o: Devices/basic/Src/power.c
	$(CC) -c Devices/basic/Src/power.c -o obj/power.o $(CFLAGS)

obj/trace.o: Libraries/trace/Src/trace.c
	$(CC) -c Libraries/trace/Src/trace.c -o obj/trace.o $(CFLAGS)

obj/axdr.o: Libraries/Convert/Src/axdr.c
	$(CC) -c Libraries/Convert/Src/axdr.c -o obj/axdr.o $(CFLAGS)

obj/bcd.o: Libraries/Convert/Src/bcd.c
	$(CC) -c Libraries/Convert/Src/bcd.c -o obj/bcd.o $(CFLAGS)

obj/button.o: Devices/sensor/Src/button.c
	$(CC) -c Devices/sensor/Src/button.c -o obj/button.o $(CFLAGS)

obj/magnetic.o: Devices/sensor/Src/magnetic.c
	$(CC) -c Devices/sensor/Src/magnetic.c -o obj/magnetic.o $(CFLAGS)

obj/keys.o: Devices/keys/Src/keys.c
	$(CC) -c Devices/keys/Src/keys.c -o obj/keys.o $(CFLAGS)

obj/flash.o: Devices/flash/Src/flash.c
	$(CC) -c Devices/flash/Src/flash.c -o obj/flash.o $(CFLAGS)

obj/eeprom.o: Devices/eeprom/Src/eeprom.c
	$(CC) -c Devices/eeprom/Src/eeprom.c -o obj/eeprom.o $(CFLAGS)

obj/meter.o: Devices/metering/Src/meter.c
	$(CC) -c Devices/metering/Src/meter.c -o obj/meter.o $(CFLAGS)

obj/crc.o: Libraries/Check/Src/crc.c
	$(CC) -c Libraries/Check/Src/crc.c -o obj/crc.o $(CFLAGS)

obj/vuart1.o: Devices/buses/Src/vuart1.c
	$(CC) -c Devices/buses/Src/vuart1.c -o obj/vuart1.o $(CFLAGS)

obj/vuart2.o: Devices/buses/Src/vuart2.c
	$(CC) -c Devices/buses/Src/vuart2.c -o obj/vuart2.o $(CFLAGS)

obj/vuart3.o: Devices/buses/Src/vuart3.c
	$(CC) -c Devices/buses/Src/vuart3.c -o obj/vuart3.o $(CFLAGS)

obj/vuart4.o: Devices/buses/Src/vuart4.c
	$(CC) -c Devices/buses/Src/vuart4.c -o obj/vuart4.o $(CFLAGS)
	
obj/viic1.o: Devices/buses/Src/viic1.c
	$(CC) -c Devices/buses/Src/viic1.c -o obj/viic1.o $(CFLAGS)

obj/viic2.o: Devices/buses/Src/viic2.c
	$(CC) -c Devices/buses/Src/viic2.c -o obj/viic2.o $(CFLAGS)

obj/viic3.o: Devices/buses/Src/viic3.c
	$(CC) -c Devices/buses/Src/viic3.c -o obj/viic3.o $(CFLAGS)
	
obj/vspi1.o: Devices/buses/Src/vspi1.c
	$(CC) -c Devices/buses/Src/vspi1.c -o obj/vspi1.o $(CFLAGS)
	
obj/vspi2.o: Devices/buses/Src/vspi2.c
	$(CC) -c Devices/buses/Src/vspi2.c -o obj/vspi2.o $(CFLAGS)

obj/stm32f0xx_adc.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_adc.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_adc.c -o obj/stm32f0xx_adc.o $(CFLAGS)

obj/stm32f0xx_can.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_can.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_can.c -o obj/stm32f0xx_can.o $(CFLAGS)

obj/stm32f0xx_cec.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_cec.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_cec.c -o obj/stm32f0xx_cec.o $(CFLAGS)

obj/stm32f0xx_comp.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_comp.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_comp.c -o obj/stm32f0xx_comp.o $(CFLAGS)

obj/stm32f0xx_crc.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_crc.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_crc.c -o obj/stm32f0xx_crc.o $(CFLAGS)

obj/stm32f0xx_crs.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_crs.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_crs.c -o obj/stm32f0xx_crs.o $(CFLAGS)

obj/stm32f0xx_dac.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_dac.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_dac.c -o obj/stm32f0xx_dac.o $(CFLAGS)

obj/stm32f0xx_dbgmcu.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_dbgmcu.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_dbgmcu.c -o obj/stm32f0xx_dbgmcu.o $(CFLAGS)

obj/stm32f0xx_dma.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_dma.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_dma.c -o obj/stm32f0xx_dma.o $(CFLAGS)

obj/stm32f0xx_exti.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_exti.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_exti.c -o obj/stm32f0xx_exti.o $(CFLAGS)

obj/stm32f0xx_flash.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_flash.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_flash.c -o obj/stm32f0xx_flash.o $(CFLAGS)

obj/stm32f0xx_gpio.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_gpio.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_gpio.c -o obj/stm32f0xx_gpio.o $(CFLAGS)

obj/stm32f0xx_i2c.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_i2c.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_i2c.c -o obj/stm32f0xx_i2c.o $(CFLAGS)

obj/stm32f0xx_iwdg.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_iwdg.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_iwdg.c -o obj/stm32f0xx_iwdg.o $(CFLAGS)

obj/stm32f0xx_misc.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_misc.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_misc.c -o obj/stm32f0xx_misc.o $(CFLAGS)

obj/stm32f0xx_pwr.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_pwr.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_pwr.c -o obj/stm32f0xx_pwr.o $(CFLAGS)

obj/stm32f0xx_rcc.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_rcc.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_rcc.c -o obj/stm32f0xx_rcc.o $(CFLAGS)

obj/stm32f0xx_rtc.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_rtc.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_rtc.c -o obj/stm32f0xx_rtc.o $(CFLAGS)

obj/stm32f0xx_spi.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_spi.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_spi.c -o obj/stm32f0xx_spi.o $(CFLAGS)

obj/stm32f0xx_syscfg.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_syscfg.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_syscfg.c -o obj/stm32f0xx_syscfg.o $(CFLAGS)

obj/stm32f0xx_tim.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_tim.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_tim.c -o obj/stm32f0xx_tim.o $(CFLAGS)

obj/stm32f0xx_usart.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_usart.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_usart.c -o obj/stm32f0xx_usart.o $(CFLAGS)

obj/stm32f0xx_wwdg.o: Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_wwdg.c
	$(CC) -c Devices\basic\Src\STM32F0xx\STM32F0xx_StdPeriph_Lib\stm32f0xx_wwdg.c -o obj/stm32f0xx_wwdg.o $(CFLAGS)

obj/stm32f0xx_it.o: Devices\basic\Src\STM32F0xx\Interrupts\stm32f0xx_it.c
	$(CC) -c Devices\basic\Src\STM32F0xx\Interrupts\stm32f0xx_it.c -o obj/stm32f0xx_it.o $(CFLAGS)