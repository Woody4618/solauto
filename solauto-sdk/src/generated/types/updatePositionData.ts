/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Option, OptionOrNullable } from '@metaplex-foundation/umi';
import {
  Serializer,
  option,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  DCASettingsInp,
  DCASettingsInpArgs,
  SolautoSettingsParametersInp,
  SolautoSettingsParametersInpArgs,
  getDCASettingsInpSerializer,
  getSolautoSettingsParametersInpSerializer,
} from '.';

export type UpdatePositionData = {
  positionId: number;
  settingParams: Option<SolautoSettingsParametersInp>;
  dca: Option<DCASettingsInp>;
};

export type UpdatePositionDataArgs = {
  positionId: number;
  settingParams: OptionOrNullable<SolautoSettingsParametersInpArgs>;
  dca: OptionOrNullable<DCASettingsInpArgs>;
};

export function getUpdatePositionDataSerializer(): Serializer<
  UpdatePositionDataArgs,
  UpdatePositionData
> {
  return struct<UpdatePositionData>(
    [
      ['positionId', u8()],
      ['settingParams', option(getSolautoSettingsParametersInpSerializer())],
      ['dca', option(getDCASettingsInpSerializer())],
    ],
    { description: 'UpdatePositionData' }
  ) as Serializer<UpdatePositionDataArgs, UpdatePositionData>;
}