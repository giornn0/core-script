use crate::utils::TitleCase;
use crate::Properties;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::{
    fs::{create_dir, create_dir_all, File},
    io::Error,
};
pub struct Config {
    section: String,
    section_plural: String,
    api_route: String,
    front_route: String,
    icon: String,
    properties: Properties,
}
impl TitleCase for &str {
    fn title(&self) -> String {
        if !self.is_ascii() || self.is_empty() {
            return String::new();
        }
        let (head, tail) = self.split_at(1);
        head.to_uppercase() + tail
    }
}
impl Config {
    pub fn new(
        section: String,
        section_plural: String,
        api_route: String,
        front_route: String,
        icon: String,
        properties: Properties,
    ) -> Config {
        Config {
            section,
            section_plural,
            api_route,
            front_route,
            icon,
            properties,
        }
    }
}
pub fn create(config: Config) -> Result<(), Error> {
    let base_path = format!("./src/app/views/{}", &config.api_route);
    let service_path = format!("./src/app/services/http/{}", &config.api_route);
    let model_name = &config.section.as_str().title();
    let plural_titled = &config.section_plural.as_str().title();
    create_dir_all(&base_path)?;
    create_dir_all(&service_path)?;
    // let templates_path = format!("{}/templates", &base_path);
    // create_dir(&templates_path)?;
    let form_path = format!("{}/pages/form", &base_path);
    create_dir_all(&form_path)?;
    let list_path = format!("{}/pages/list", &base_path);
    create_dir(&list_path)?;
    // let modal_path = format!("{}/pages/modal", &base_path);
    // create_dir(&modal_path)?;
    let show_path = format!("{}/pages/show", &base_path);
    create_dir(&show_path)?;
    let mut form_html = File::create(format!("{}/form.component.html", &form_path)).unwrap();
    write_form_html(&mut form_html, &config.section)?;
    let mut form_ts = File::create(format!("{}/form.component.ts", &form_path)).unwrap();
    write_form_ts(&mut form_ts, &config.section, model_name)?;

    let mut list_html = File::create(format!("{}/list.component.html", &list_path)).unwrap();
    write_list_html(&mut list_html, &config.section_plural)?;
    let mut list_ts = File::create(format!("{}/list.component.ts", &list_path)).unwrap();
    write_list_ts(
        &mut list_ts,
        model_name,
        &config.section,
        plural_titled,
        &config.section_plural,
        &config.api_route,
    )?;

    let mut show_html = File::create(format!("{}/show.component.html", &show_path)).unwrap();
    write_show_html(&mut show_html)?;
    let mut show_ts = File::create(format!("{}/show.component.ts", &show_path)).unwrap();
    write_show_ts(&mut show_ts, model_name, &config.section)?;

    // let mut modal_html = File::create(format!("{}/modal.component.html", &modal_path)).unwrap();
    // write_modal_html(&mut modal_html, &config.section)?;
    // let mut modal_ts = File::create(format!("{}/modal.component.ts", &modal_path)).unwrap();
    // write_modal_ts(&mut modal_ts, model_name, &config.section)?;

    let mut routing = File::create(format!(
        "{}/{}-routing.module.ts",
        &base_path, &config.api_route
    ))?;
    write_routing(
        &mut routing,
        model_name,
        &config.section_plural,
        plural_titled,
        &config.section,
    )?;

    let mut module = File::create(format!("{}/{}.module.ts", &base_path, &config.api_route))?;
    write_module(
        &mut module,
        &config.section_plural,
        plural_titled,
        &config.api_route,
    )?;

    let mut resolver = File::create(format!(
        "{}/{}.resolver.ts",
        &service_path, &config.section_plural
    ))?;
    write_resolvers(&mut resolver, model_name, plural_titled)?;

    let mut service = File::create(format!(
        "{}/{}.service.ts",
        &service_path, &config.api_route
    ))?;
    write_model_service(
        &mut service,
        model_name,
        &config.section,
        &config.section_plural,
        plural_titled,
    )?;

    update_nav(plural_titled, model_name, &config.icon)?;
    update_routes(model_name, plural_titled, &config.api_route)?;
    update_route_enums(model_name, &config.front_route, &config.api_route)?;

    // let mut form_interface = File::create(format!("{}/form.interface.ts", &templates_path))?;
    // write_form_interface(&mut form_interface, model_name)?;

    // let mut form_template = File::create(format!("{}/form.template.ts", &templates_path))?;
    // write_form_template(&mut form_template, &config.properties)?;
    //
    // let mut list_template = File::create(format!("{}/list.temp.functions.ts", &templates_path))?;
    // write_list_template(
    //     &mut list_template,
    //     model_name,
    //     &config.section,
    //     &config.icon,
    // )?;

    let mut model_ts = File::create(format!(
        "./src/app/shared/models/{}.model.ts",
        &config.section
    ))?;
    write_model_ts(&mut model_ts, model_name, &config.properties)?;
    Ok(())
}

fn write_model_ts(file: &mut File, model: &str, properties: &Properties) -> Result<(), Error> {
    writeln!(
        file,
        r#"export interface {model} {{
  id : number;
  name : string;
  /*
   {:?}
  */
  created_at : Date;
  updated_at: Date;
}}
"#,
        properties
    )
}

fn write_list_template(
    file: &mut File,
    model: &str,
    section: &str,
    icon: &str,
) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ {model} }} from "../../../shared/models/{section}.model";
import {{
  MainListTemplate,
  ListTemplate,
  SpecialBlock,
  ButtonAction,
  Color,
  ButtonType,
}} from "../../../shared/models/basic/template.list.model";
import {{ FontDataRender }} from "../../../shared/models/basic/fonts.template.const";

export const template: MainListTemplate = {{
  headIcon: "fas {icon} fa-2x ms-1 mt-1",
}};

export function setData(obj: {model}): ListTemplate {{
  const  btnDestacado: ButtonAction = {{
    name: obj.id ? "Destacado" : "No destacado",
    icon: obj.id ? "fas fa-star" : '',
    color: obj.id ? Color.warning : Color.medium,
    type: ButtonType.chip,
    tooltip: obj.id ? "No destacar" : "Destacar",
    values: {{ name: "isDestacado", status: obj.id, id: obj.id }},
  }};
  return {{
    nombreForDelete: `${{obj.id}}`,
    avatar: "{icon}",
    alerted:{{
      message: 'Cod. '+obj.id,
      color: Color.primary
    }},
    header: {{ header: "${{obj.id}}", fontEnum: FontDataRender.Header }},
    personalEdit: true,
    personalDelete: true,
    id: obj.id,
    specialButtons: [btnDestacado],
    status: obj.id?`${{obj.id ? "true" : "false"}}`: undefined,
    properties: [],
  }};
}}
export function showSetData(obj: {model}): ListTemplate{{
    return {{  }} as ListTemplate
}}
export function setValueQueryPlus(obj: {model}) {{
  return {{  }};
}}"#
    )
}

fn write_form_template(file: &mut File, properties: &Properties) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Validators }} from "@angular/forms";
import {{
  InputTemplate,
  InputTypes,
}} from "../../../shared/models/basic/template.form.model";
import {{ InitialState }} from "../../../root/root-form/inputs/inputs.component";
/*
{:?}
*/

export const initialState: {{ [x: string]: InitialState }} = {{
  name: {{ value: null, disabled: false , validators: [Validators.required] }},
}};


export const campsTemplate: Array<InputTemplate[]> = [
  [
    {{
      name: "name",
      class: "col-sm-6",
      label: "Nombre Visible",
      required: true,
      typeinput: "text",
    }},
  ]
];"#,
        properties
    )
}

fn write_form_interface(file: &mut File, model: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ FormInterface }} from "../../../root/root-form/form/form.component";

export interface {model}Form extends FormInterface{{
}}
"#
    )
}

fn update_route_enums(model: &str, front_route: &str, api_route: &str) -> Result<(), Error> {
    let mut route_enums = OpenOptions::new()
        .write(true)
        .open("./src/app/constants/routes.ts")?;
    route_enums.seek(SeekFrom::End(-4))?;
    writeln!(
        route_enums,
        r#"
  {model} = "{front_route}",
  {model}Api = "{api_route}",
}}"#
    )
}

fn update_routes(model: &str, plural_titled: &str, api_route: &str) -> Result<(), Error> {
    let mut routes = OpenOptions::new()
        .write(true)
        .open("./src/app/_routes.ts")?;
    routes.seek(SeekFrom::End(-4))?;
    writeln!(
        routes,
        r#"
  {{
    path: RoutesEnum.{model},
    loadChildren: () =>
      import("./views/{api_route}/{api_route}.module").then(
        (m) => m.{plural_titled}Module
      ),
  }},
];"#
    )
}

fn update_nav(plural_titled: &str, model: &str, icon: &str) -> Result<(), Error> {
    let mut nav = OpenOptions::new().write(true).open("./src/app/_nav.ts")?;
    nav.seek(SeekFrom::End(-4))?;
    writeln!(
        nav,
        r#"
  {{
    title: true,
    name: RoutesEnum.{model},
    class: "strong-font",
    children: [
      {{
        name: RoutesEnum.{model},
        url: `${{RoutesEnum.{model}}}/listar`,
        icon: "fa {icon} scaling",
        class: "strong-font",
        active: RoutesEnum.{model},
      }},
    ],
  }},
];"#
    )
}

fn write_resolvers(file: &mut File, model: &str, plural_titled: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Injectable }} from '@angular/core';
import {{Resolve,
  RouterStateSnapshot,
  ActivatedRouteSnapshot
}} from '@angular/router';
import {{ Observable }} from 'rxjs';
import {{ RoutesEnum }} from '../../../constants/routes';
import {{ RootService }} from '../root.service';

@Injectable({{
  providedIn: 'root'
}})
export class All{plural_titled}Resolver implements Resolve<unknown> {{
  constructor(private rootService:RootService){{}}
  resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Observable<unknown> {{
    return this.rootService.getAll(RoutesEnum.{model}Api)
  }}
}}
@Injectable({{
  providedIn: 'root'
}})
export class {model}Resolver implements Resolve<unknown> {{
  constructor(private rootService:RootService){{}}
  resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Observable<unknown> {{
    const{{id}} = route.params
    return this.rootService.getOne(RoutesEnum.{model}Api,id)
  }}
}}
@Injectable({{
  providedIn: 'root'
}})
export class {plural_titled}Resolver implements Resolve<unknown> {{
  constructor(private rootService:RootService){{}}
  resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Observable<unknown> {{
    const {{take,page,search}} = route.queryParams
    return this.rootService.index('productos/rubros',page,take,search)
  }}
}}
"#
    )
}

fn write_routing(
    file: &mut File,
    model: &str,
    section_plural: &str,
    plural_titled: &str,
    section: &str,
) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ NgModule }} from "@angular/core";
import {{ Routes, RouterModule }} from "@angular/router";
//import {{ Modal{plural_titled}Resolver }} from "../../constants/resolvers";
import {{ RoutesEnum }} from "../../constants/routes";
import {{ FormComponent }} from "./pages/form/form.component";
import {{ ListComponent }} from "./pages/list/list.component";
import {{ ShowComponent }} from "./pages/show/show.component";

const section = RoutesEnum.{model};
const apiSection = RoutesEnum.{model}Api;

const routes: Routes = [
  {{
    path: "",
    data: {{
      title: "{plural_titled}",
    }},
    children: [
      {{
        path: "listar",
        component: ListComponent,
        data: {{
          section,
          apiSection,
          modalComponent: FormComponent,
          showComponent: ShowComponent,
      //  resolvers:{{
      //    {section_plural}: Modal{plural_titled}Resolver,
      //  }}
        }},
      }},
    ],
  }},
];

@NgModule({{
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
}})
export class {plural_titled}RoutingModule {{}}
export const routingComponents = [FormComponent, ListComponent, ShowComponent];
"#
    )
}
fn write_module(
    file: &mut File,
    section_plural: &str,
    plural_titled: &str,
    api_route: &str,
) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ CommonModule }} from '@angular/common';
import {{ NgModule }} from '@angular/core';
import {{ ReactiveFormsModule, FormsModule }} from '@angular/forms';
import {{ PaginationModule }} from 'ngx-bootstrap/pagination';
import {{ SharedModule }} from '../../shared/shared.module';
import {{ {plural_titled}RoutingModule, routingComponents }} from './{api_route}-routing.module';
import {{ RootModule }} from '../../root/root.module';
import {{ MatPaginatorModule }} from '@angular/material/paginator';
import {{ HeaderListadoModule }} from '../../shared/components/header-listado/header-listado.module';
import {{ MatDialogModule }} from '@angular/material/dialog';
import {{ MatFormFieldModule }} from '@angular/material/form-field';
import {{ MatInputModule }} from '@angular/material/input';
import {{ MatButtonModule }} from '@angular/material/button';
import {{ StatusModule }} from '../../shared/components/buttons/status/status.module';
import {{ EditModule }} from '../../shared/components/buttons/edit/edit.module';
import {{ DeleteModule }} from '../../shared/components/buttons/delete/delete.module';

@NgModule({{
  imports: [
    {plural_titled}RoutingModule,
    ReactiveFormsModule,
    CommonModule,
    PaginationModule,
    FormsModule,
    SharedModule,
    RootModule,
    MatPaginatorModule,
    MatDialogModule,
    HeaderListadoModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    StatusModule,
    EditModule,
    DeleteModule
  ],
  declarations: routingComponents
}})
export class {plural_titled}Module {{ }}
"#
    )
}
fn write_form_html(file: &mut File, section: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"<div class="container">
    <h6 mat-dialog-title class="text-primary">{{{{title}}}}</h6>
    <mat-dialog-content>
      <form [formGroup]="form">
        <div class="row w-100 p-0 m-0">
          <div class="col-12">
            <mat-form-field class="w-100" appearance="outline">
              <mat-label> Nombre </mat-label>
              <input matInput formControlName="name" />
            </mat-form-field>
          </div>
        </div>
      </form>
    </mat-dialog-content>
    <div
      mat-dialog-actions
      align="center"
      class="m-0 p-0 d-flex justify-content-evenly"
    >
      <button
        mat-flat-button
        color="warn"
        type="button"
        (click)="close()"
      >
        Cancelar
      </button>
  
      <button mat-flat-button color="primary" type="button" (click)="submit()" [disabled]="form.invalid">
        Guardar
      </button>
    </div>
  </div>"#
    )
}
fn write_form_ts(file: &mut File, section: &str, model: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ HttpErrorResponse }} from "@angular/common/http";
import {{ Component, Inject, OnInit }} from "@angular/core";
import {{ Validators, FormBuilder }} from "@angular/forms";
import {{ MatDialogRef, MAT_DIALOG_DATA }} from "@angular/material/dialog";
import {{ MatSnackBar }} from "@angular/material/snack-bar";
import {{ Observable }} from "rxjs";
import {{ RoutesEnum }} from "../../../../constants/routes";
import {{ RootService }} from "../../../../services/http/root.service";
import {{ {model} }} from "../../../../shared/models/{section}.model";

@Component({{
  templateUrl: "form.component.html",
  styleUrls: ["../../../../app.component.css"],
}})
export class FormComponent implements OnInit {{
  form = this.fb.group({{
    name: ["", Validators.required],
  }});
  {section}?: {model};

  constructor(
    @Inject (MAT_DIALOG_DATA)public data:any,
    private rootService: RootService,
    private snackbar: MatSnackBar,
    private fb: FormBuilder,
    private dialog: MatDialogRef<FormComponent>
  ) {{
    this.getEntity();
  }}

  protected getEntity(): void {{
    if (this.data) {{
      const {{ entityForEdit }} = this.data;
      this.{section} = entityForEdit;
    }}
  }}
  ngOnInit(): void {{
    if (this.{section}) {{
      this.form.reset({{
        name: this.{section}.name,
      }});
    }}
  }}

  close(): void {{
    this.dialog.close();
  }}

  submit(): void {{
    // console.log(this.form.value);
    if (this.{section}) {{
      this.edit();
    }} else {{
      this.create();
    }}
  }}
  create() {{
    this.rootService
      .create(RoutesEnum.{model}Api, this.form.value)
      .subscribe({{
        next: (resp) => {{
          this.snackbar.open(`{model} creado correctamente.`, "Ok", {{
            duration: 2500,
            panelClass: ["primary-snackbar"],
            verticalPosition: "top",
            horizontalPosition: "end",
          }});
          this.dialog.close({{ listChange: true }});
        }},
        error: (err: HttpErrorResponse) => {{
          this.snackbar.open(
            "No pudimos crear el {model}. " + `(${{err.status}})`,
            "Ok",
            {{
              panelClass: ["error-snackbar"],
              verticalPosition: "top",
              horizontalPosition: "end",
            }}
          );
          this.dialog.close({{ listChange: false }});
        }},
      }});
  }}
  edit(): void {{
    if (this.{section}) {{
      this.rootService
        .update(
          RoutesEnum.{model}Api,
          {{ ...this.form.value, id: this.{section}.id }},
          this.{section}.id
        )
        .subscribe({{
          next: (resp) => {{
            this.snackbar.open(
              `${{this.form.get("name")?.value}} editado correctamente.`,
              "Ok",
              {{
                duration: 2500,
                panelClass: ["primary-snackbar"],
                verticalPosition: "top",
                horizontalPosition: "end",
              }}
            );
            this.dialog.close({{ listChange: true }});
          }},
          error: (err: HttpErrorResponse) => {{
            this.snackbar.open(
              "No se completó el proceso de editado. " + `(${{err.status}})`,
              "Ok",
              {{
                panelClass: ["error-snackbar"],
                verticalPosition: "top",
                horizontalPosition: "end",
              }}
            );
            this.dialog.close({{ listChange: false }});
          }},
        }});
    }}
  }}

  get title(): string {{
    return this.{section}
      ? `Editando ${{this.{section}.name}}`
      : "Creando nuevo {section}";
  }}
}}
"#
    )
}

fn write_list_html(file: &mut File, plural: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"<div #topScrollAnchor></div>
<div class="row wrapp-all animated fadeIn">
  <div class="col-sm-12">
    <div class="row">
      <div class="col-12 col-lg-5 mb-2">
        <label class="form-title pt-0">
          <span class="d-none d-xl-inline"> Listado de </span>
          {{{{ title }}}}
        </label>
      </div>

      <div class="col-12 col-lg-7 mb-2">
        <div class="row">
          <div class="col-12 col-sm-6 mb-2">
            <app-shared-searcher (valueSearch)="search($event)">
            </app-shared-searcher>
          </div>
          <div class="col-12 col-sm-6">
            <app-header-listado></app-header-listado>
          </div>
        </div>
      </div>

      <div class="col-12">
        <simple-card
          *ngFor="let ent of {plural}List$ | async; let i = index"
          [id]="'card' + i"
          [ngClass]="ent.status === false ? 'fade-out-left' : ''"
        >
          <div
            id="main-row"
            class="row bg-white text-medium mb-2 p-1 pt-2 ps-2 pe-0 rounded"
          >
            <div class="col-auto ps-0 pe-0">
              <h6
                class="breakWord-blank m-0 text-black-75 w-100"
                style="font-size: 0.95rem"
              >
                {{{{ ent.name | titlecase }}}}
              </h6>
            </div>

            <div class="col-sm-12 ps-0">
              <div class="row w-100 justify-content-end">
                <div class="col-auto">
                  <div class="row">
                    <div class="col-auto" *ngIf="ent?.created_at">
                      <small class="m-0 p-0 text-black-75 fst-italic"
                        >Creado: </small
                      ><small class="m-0 p-0 text-medium fw-bold">{{{{
                        ent?.created_at | date: "medium"
                      }}}}</small>
                    </div>
                    <div class="col-auto" *ngIf="ent?.updated_at">
                      <small class="m-0 p-0 text-black-75 fst-italic"
                        >Modificado: </small
                      ><small class="m-0 p-0 text-medium fw-bold">{{{{
                        ent?.updated_at | date: "medium"
                      }}}}</small>
                    </div>
                  </div>
                </div>
                <div
                  class="col d-flex justify-content-end align-items-center p-0"
                >
                  <div class="row d-flex justify-content-end">
                    <div class="col-1">
                      <status-button
                        [id]="ent.id"
                        [statusToggle]="{{ status: ent.status }}"
                        [apiSection]="pathDelete ?? apiSection ?? ''"
                        [index]="i"
                        (switch)="disableCard($event)"
                      ></status-button>
                    </div>
                    <div class="col-1">
                      <edit-button
                        [id]="ent.id"
                        [form]="formComponentEdit"
                      ></edit-button>
                    </div>
                    <div class="col-1">
                      <delete-button
                        [config]="{{
                          id: ent.id,
                          nameToDelete: ent.name,
                          section: apiRoute 
                        }}"
                        (listChange)="listChange($event)"
                      ></delete-button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </simple-card>
      </div>

      <div class="col-12" *ngIf="({plural}List$ | async )?.length">
        <mat-paginator
          color="primary"
          [length]="totalItems"
          [pageIndex]="currentPage"
          [pageSize]="itemsPerPage"
          [pageSizeOptions]="defaultTakeOptions"
          aria-label="Elegir página"
          showFirstLastButtons
          (page)="changePage($event)"
        >
        </mat-paginator>
      </div>
    </div>
  </div>
</div>
"#
    )
}
fn write_list_ts(
    file: &mut File,
    model: &str,
    section: &str,
    plural_titled: &str,
    plural: &str,
    api_route: &str,
) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ ComponentType }} from "@angular/cdk/portal";
import {{ Component, OnInit }} from "@angular/core";
import {{ MatDialog }} from "@angular/material/dialog";
import {{ ActivatedRoute, Router }} from "@angular/router";
import {{ delay, filter, Observable, switchMap }} from "rxjs";
import {{ InteractionsId }} from "../../../../constants/interactions";
import {{ RoutesEnum }} from "../../../../constants/routes";
import {{ UserRole }} from "../../../../constants/userRoles";
import {{ ListBaseComponent }} from "../../../../root/root-list/list-base/list-base.component";
import {{ {model}Service }} from "../../../../services/http/{api_route}/{api_route}.service";
import {{ RootService }} from "../../../../services/http/root.service";
import {{ InteractionService, InteractionSource }} from "../../../../services/interaction/interaction.service";
import {{ LoginService }} from "../../../../services/login/login.service";
import {{ ModalService }} from "../../../../services/modals/modals.service";
import {{ ArrayResponse }} from "../../../../shared/models/basic/res.array.model";
import {{ FormComponent }} from "../form/form.component";
import {{ {model} }} from "../../../../shared/models/{section}.model";

@Component({{
  selector: "app-list",
  templateUrl: "./list.component.html",
  styleUrls: ["../../../../app.component.css"],
}})
export class ListComponent extends ListBaseComponent implements OnInit {{
  constructor(
    private logService: LoginService,
    activatedRoute: ActivatedRoute,
    rootService: RootService,
    router: Router,
    modalService: ModalService,
    interaction: InteractionService,
    dialog: MatDialog,
    private {section}Service: {model}Service
  ) {{
    super(activatedRoute, router, rootService, modalService, interaction, dialog);
  }}
  userRole?: UserRole;
  userId?: string|null;
  {plural}List$?: Observable<{model}[]>;
  formComponentEdit: ComponentType<FormComponent> = FormComponent;
  routeEnum = RoutesEnum.{model}Api;

  ngOnInit() {{
    this.{plural}List$ = this.{section}Service.{plural};
    this.subscriptions.add(
      this.activatedRoute.queryParams.subscribe((query)=>{{
        const {{page, take, search}} = query;
        this.queryParams.queryParams={{page, take, search}};
        this.rootService.index<{model}>(this.apiSection!, page, take, search)
        .subscribe((res:ArrayResponse<{model}>)=>{{
          this.{section}Service.set{plural_titled}(res.data);
          this.setPage(res);
        }})
      }})
    );
    this.subscriptions.add(
      this.interactionService.interaction
        .pipe(
          filter(
            (e) =>
              (e.source === InteractionSource.button &&
              e.id === InteractionsId.listChange) || (e.id === InteractionsId.RootForm)
          ),
          delay(800)
        )
        .subscribe((res) => this.listChange())
    );
  }}

  listChange(e?: any): void {{
    this.rootService
      .index<{model}>(this.apiSection ?? '')
      .pipe(switchMap(async (res) => this.{section}Service.set{plural_titled}(res.data)))
      .subscribe();
    this.dialog.closeAll();
  }}
  
  disableCard(e: any): void {{
    if (e.status === 0) {{
      document
        .getElementById("card" + e.i)
        ?.classList.replace("fade-in", "fade-out-left");
    }} else {{
      document
        .getElementById("card" + e.i)
        ?.classList.replace("fade-out-left", "fade-in");
    }}
  }}
}}
"#
    )
}
fn write_show_html(file: &mut File) -> Result<(), Error> {
    writeln!(
        file,
        r#"<app-root-show
    [forShow]="showing"
>
</app-root-show>
"#
    )
}
fn write_show_ts(file: &mut File, model: &str, section: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Component, Inject }} from "@angular/core";
import {{ MAT_DIALOG_DATA }} from "@angular/material/dialog";
import {{ ModalShowBaseComponent }} from "../../../../root/root-show/modal-show-base/modal-show-base.component";
import {{ DataModalShow }} from "../../../../shared/models/basic/data.routes.model";
import {{ {model} }} from "../../../../shared/models/{section}.model";

@Component({{
    templateUrl: "./show.component.html",
  }})
  export class ShowComponent extends ModalShowBaseComponent {{
    constructor(
    @Inject(MAT_DIALOG_DATA) data: DataModalShow<{model}>
    ){{
        super(data)
    }}
  }}
"#
    )
}
fn write_modal_html(file: &mut File, section: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"<mat-dialog-content>
    <app-root-form 
        #form
        [template]="campsTemplate!"
        [title]="title"
        [forEdit]="{section}ForEdit"
        [initialStateControls]="initialState"
        [selectOptions]="optionsPassed"
        [section]="section!"
        [APIroute]="apiSection!"
        [isModal]="true"
        (close)="close($event)"
    >
    </app-root-form>
</mat-dialog-content>"#
    )
}
fn write_modal_ts(file: &mut File, model: &str, section: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Component, Inject, OnInit, AfterViewInit }} from "@angular/core";
import {{ ChangeDetectorRef }} from "@angular/core";
import {{ LoginService }} from "../../../../services/login/login.service";
import {{ MatDialog, MatDialogRef, MAT_DIALOG_DATA }} from "@angular/material/dialog";
import {{ ModalFormBaseComponent }} from "../../../../root/root-form/modal-form-base/modal-form-base.component";
import {{ DataEditForm }} from "../../../../shared/models/basic/data.routes.model";
import {{ RootService }} from "../../../../services/http/root.service";
import {{ InteractionService }} from "../../../../services/interaction/interaction.service";
import {{ {model} }} from "../../../../shared/models/{section}.model";
import {{ {model}Form }} from "../../templates/form.interface";

@Component({{
  templateUrl: "modal.component.html",
  styleUrls: ["../../../../app.component.css"],
}})
export class ModalComponent extends ModalFormBaseComponent implements {model}Form, OnInit, AfterViewInit {{
  constructor(
    private cdr: ChangeDetectorRef,
    dialogRef: MatDialogRef<ModalComponent>,
    logService: LoginService,
    interaction: InteractionService,
    dialog: MatDialog,
    rootService: RootService,
    @Inject(MAT_DIALOG_DATA) data: DataEditForm
  ) {{
    super(dialogRef, logService, data, interaction, dialog, rootService);
  }}
  override activoInactivoOptions: any[] = [
{{value: "destacado", description: "Destacado" }},
{{value: "estado", description: "Activo" }},
  ];
  
{section}ForEdit?: {model};

  ngOnInit() {{
    this.setStartingValues();
  }}

  setStartingValues(){{
    // const {{ {section} }} = this.data.resolvers!;
    this.optionsPassed["opciones"] = this.activoInactivoOptions; // Cada Campo usa su name como respectiva key
    this.{section}ForEdit = this.data.entityForEdit;
    if (this.{section}ForEdit) {{
    // this.{section}ForEdit.opciones = [];
    // if (this.{section}ForEdit?.estado){{
    //   this.{section}ForEdit.opciones.push("estado");
    // }}
    }}
  }}
  
  ngAfterViewInit(): void {{
    //ACA HACER USO DEL FORM
    this.inputsControls = this.form?.inputs.toArray();
  }}
}}
"#
    )
}
fn write_model_service(
    file: &mut File,
    model: &str,
    section: &str,
    plural: &str,
    plural_titled: &str,
) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Injectable }} from "@angular/core";
import {{ BehaviorSubject, Observable, Subject, tap }} from "rxjs";
import {{ RoutesEnum }} from "../../../constants/routes";
import {{ {model} }} from "../../../shared/models/{section}.model";
import {{ RootService }} from "../root.service";

@Injectable({{
  providedIn: "root",
}})
export class {model}Service {{
  #{section}: Subject<{model}> = new Subject<{model}>();
  #{plural}: BehaviorSubject<{model}[]> = new BehaviorSubject<{model}[]>([]);

  constructor(private rootService: RootService) {{
    this.#{section}.subscribe((col) =>
      this.#{plural}.next([...this.#{plural}.value, col])
    );
  }}

  get {section} 
  (): Observable<{model}> {{
    return this.#{section}.asObservable();
  }}

  get {plural}(): Observable<{model}[]> {{
    return this.#{plural}.asObservable();
  }}

  set{plural_titled}({plural}: {model}[]): void {{
    this.#{plural}.next({plural});
  }}

  public nurture{plural_titled}(): Observable<{model}[]> {{
    return this.rootService
      .getAllActive<{model}[]>(RoutesEnum.{model}Api)
      .pipe(tap(({plural}) => this.set{plural_titled}({plural})));
  }}

  emit{model}({section}: {model}): void {{
    this.#{section}.next({section});
  }}
}}
"#
    )
}
