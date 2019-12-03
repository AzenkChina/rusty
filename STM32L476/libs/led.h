/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __LED_H__
#define __LED_H__

/* Includes ------------------------------------------------------------------*/
#include "stdint.h"

/* Exported types ------------------------------------------------------------*/
struct __led
{
	void (*init)();
	void (*suspend)();
	void (*on)();
	void (*off)();
};

/* Exported constants --------------------------------------------------------*/
/* Exported macro ------------------------------------------------------------*/

/* Exported function prototypes ----------------------------------------------*/
extern const struct __led led;

#endif /* __LED_H__ */
