#include <ace/ace_engine.h>
#include <ability_loader.h>
#include <hilog/log.h>

using namespace OHOS::Ace;

// 应用程序主入口函数
int main(int argc, char **argv)
{
    // 初始化日志系统
    OHOS::HiviewDFX::HiLog::Info({0, 0}, "EntryAbility main function started");

    // 初始化ACE引擎
    OHOS::Ace::Platform::AceEngine::Get().Initialize();
    
    // 加载Ability
    OHOS::AppExecFwk::AbilityLoader::GetInstance()->LoadAllAbilities();
    
    // 运行事件循环
    OHOS::Ace::Platform::AceEngine::Get().Run();
    
    return 0;
}