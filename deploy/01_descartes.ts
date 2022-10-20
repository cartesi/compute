import { HardhatRuntimeEnvironment } from "hardhat/types";
import { DeployFunction } from "hardhat-deploy/types";

const func: DeployFunction = async (hre: HardhatRuntimeEnvironment) => {
    const { deployments, getNamedAccounts } = hre;
    const { deploy, get } = deployments;
    const { deployer } = await getNamedAccounts();

    const Merkle = await get("Merkle");
    const Logger = await get("Logger");
    const VGInstantiator = await get("VGInstantiator");
    const Step = await get("Step");
    await deploy("CartesiCompute", {
        from: deployer,
        log: true,
        libraries: {
            Merkle: Merkle.address,
        },
        args: [Logger.address, VGInstantiator.address, Step.address],
    });
};

export default func;
export const tags = ["CartesiCompute"];
