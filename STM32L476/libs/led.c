/**
 * @brief		
 * @details		
 * @date		2016-08-16
 **/

/* Includes ------------------------------------------------------------------*/
#include "led.h"

/* Private typedef -----------------------------------------------------------*/
/* Private define ------------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private variables ---------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
/* Private functions ---------------------------------------------------------*/
static void led_init()
{
	
}

static void led_suspend()
{
	
}

static void led_on()
{
	
}

static void led_off()
{
	
}



const struct __led led = 
{
	.init		= led_init,
	.suspend	= led_suspend,
	.on			= led_on,
	.off		= led_off,
};

