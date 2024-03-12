use rust_web_ui::*;

pub struct Example3;

pub struct Example3Props {}

impl Default for Example3Props {
	fn default() -> Example3Props {
		Example3Props { }
	}
}

impl Widget<'_> for Example3 {
	type Props = Example3Props;

	fn render(mut ctx: Ctx<'_>, props: Example3Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Color(Color::Fg(11))]);
			props.set_attribute("style", "min-height: 192px");
			
			props.child("1", Dynamic).run("nav", |mut props| {
				props.set_attribute("x-data", "{ open: false }");
				props.styles(&[Style::Color(Color::Fg(89))]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(8)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("relative"), Style::Flex, Style::Width(Size::Exact(64)), Style::ItemsCenter, Style::JustifyBetween]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("left-0"), Style::Flex, Style::ItemsCenter, Style::OnScreen(Screen::Small, &[Style::Noop("hidden")])]);
							
							//  Mobile menu button
							props.child("3", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Padding(Size::Exact(8)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::Color(Color::Fg(78))]), Style::OnHover(&[Style::Noop("text-white")]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-white")])]);
								props.set_attribute("aria-controls", "mobile-menu");
								props.set_attribute("@click", "open = !open");
								props.set_attribute("aria-expanded", "false");
								props.set_attribute("x-bind:aria-expanded", "open.toString()");
								
								props.child("1", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("absolute"), Style::Noop("-inset-0.5")]);
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("Open main menu"));
								});
								props.child("5", Icon).run(|mut props| {
									props.style(&[Style::Block, Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
								});
								props.child("7", Icon).run(|mut props| {
									props.style(&[Style::Noop("hidden"), Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("flex-1"), Style::ItemsCenter, Style::JustifyCenter, Style::OnScreen(Screen::Small, &[Style::Noop("items-stretch")]), Style::OnScreen(Screen::Small, &[Style::Noop("justify-start")])]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::Noop("flex-shrink-0"), Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("img", |mut props| {
									props.styles(&[Style::Width(Size::Exact(32)), Style::Noop("w-auto")]);
									props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500");
									props.set_attribute("alt", "Your Company");
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Small, &[Style::MarginLeft(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::Block])]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::SpaceX(Size::Exact(16))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Color(Color::Fg(100)), Style::Noop("text-white"), Style::Noop("rounded-md"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium")]);
										props.set_attribute("x-state:on", "Current");
										props.set_attribute("x-state:off", "Default");
										props.set_attribute("aria-current", "page");
										props.set_attribute("x-state-description", r#"Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
										
										props.child("0", Label).run(|props| props.text("Dashboard"));
									});
									props.child("3", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::TextColor(Color::Fg(33)), Style::OnHover(&[Style::Color(Color::Fg(78))]), Style::OnHover(&[Style::Noop("text-white")]), Style::Noop("rounded-md"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium")]);
										props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
										
										props.child("0", Label).run(|props| props.text("Team"));
									});
									props.child("5", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::TextColor(Color::Fg(33)), Style::OnHover(&[Style::Color(Color::Fg(78))]), Style::OnHover(&[Style::Noop("text-white")]), Style::Noop("rounded-md"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium")]);
										props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
										
										props.child("0", Label).run(|props| props.text("Projects"));
									});
									props.child("7", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::TextColor(Color::Fg(33)), Style::OnHover(&[Style::Color(Color::Fg(78))]), Style::OnHover(&[Style::Noop("text-white")]), Style::Noop("rounded-md"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium")]);
										props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
										
										props.child("0", Label).run(|props| props.text("Calendar"));
									});
								});
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(Size::Exact(8)), Style::OnScreen(Screen::Small, &[Style::Noop("static")]), Style::OnScreen(Screen::Small, &[Style::Noop("inset-auto")]), Style::OnScreen(Screen::Small, &[Style::MarginLeft(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::PaddingRight(Size::Exact(0))])]);
							
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Noop("relative"), Style::Noop("rounded-full"), Style::Color(Color::Fg(89)), Style::Padding(Size::Exact(4)), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::Noop("text-white")]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-white")]), Style::OnFocus(&[Style::Noop("ring-offset-2")]), Style::OnFocus(&[Style::Noop("ring-offset-gray-800")])]);
								
								props.child("1", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("absolute"), Style::Noop("-inset-1.5")]);
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("View notifications"));
								});
								props.child("5", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
								});
							});
							//  Profile dropdown 
							props.child("5", Dynamic).run("div", |mut props| {
								props.set_attribute("x-data", "Components.menu({ open: false })");
								props.set_attribute("x-init", "init()");
								props.set_attribute("@keydown.escape.stop", "open = false; focusButton()");
								props.set_attribute("@click.away", "onClickAway($event)");
								props.styles(&[Style::Noop("relative"), Style::MarginLeft(Size::Exact(12))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.child("1", Dynamic).run("button", |mut props| {
										props.set_attribute("type", "button");
										props.styles(&[Style::Noop("relative"), Style::Flex, Style::Noop("rounded-full"), Style::Color(Color::Fg(89)), Style::Noop("text-sm"), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-white")]), Style::OnFocus(&[Style::Noop("ring-offset-2")]), Style::OnFocus(&[Style::Noop("ring-offset-gray-800")])]);
										props.set_attribute("id", "user-menu-button");
										props.set_attribute("x-ref", "button");
										props.set_attribute("@click", "onButtonClick()");
										props.set_attribute("@keyup.space.prevent", "onButtonEnter()");
										props.set_attribute("@keydown.enter.prevent", "onButtonEnter()");
										props.set_attribute("aria-expanded", "false");
										props.set_attribute("aria-haspopup", "true");
										props.set_attribute("x-bind:aria-expanded", "open.toString()");
										props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
										props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("absolute"), Style::Noop("-inset-1.5")]);
										});
										props.child("3", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("sr-only")]);
											
											props.child("0", Label).run(|props| props.text("Open user menu"));
										});
										props.child("5", Dynamic).run("img", |mut props| {
											props.styles(&[Style::Width(Size::Exact(32)), Style::Width(Size::Exact(32)), Style::Noop("rounded-full")]);
											props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
											props.set_attribute("alt", "");
										});
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.set_attribute("x-show", "open");
									props.set_attribute("x-transition:enter", "transition ease-out duration-100");
									props.set_attribute("x-transition:enter-start", "transform opacity-0 scale-95");
									props.set_attribute("x-transition:enter-end", "transform opacity-100 scale-100");
									props.set_attribute("x-transition:leave", "transition ease-in duration-75");
									props.set_attribute("x-transition:leave-start", "transform opacity-100 scale-100");
									props.set_attribute("x-transition:leave-end", "transform opacity-0 scale-95");
									props.styles(&[Style::Noop("absolute"), Style::Noop("right-0"), Style::Noop("z-10"), Style::MarginTop(Size::Exact(8)), Style::Width(Size::Exact(192)), Style::Noop("origin-top-right"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingY(Size::Exact(4)), Style::Noop("shadow-lg"), Style::Noop("ring-1"), Style::Noop("ring-black"), Style::Noop("ring-opacity-5"), Style::OnFocus(&[Style::Noop("outline-none")])]);
									props.set_attribute("x-ref", "menu-items");
									props.set_attribute("x-description", "Dropdown menu, show/hide based on menu state.");
									props.set_attribute("x-bind:aria-activedescendant", "activeDescendant");
									props.set_attribute("role", "menu");
									props.set_attribute("aria-orientation", "vertical");
									props.set_attribute("aria-labelledby", "user-menu-button");
									props.set_attribute("tabindex", "-1");
									props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
									props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
									props.set_attribute("@keydown.tab", "open = false");
									props.set_attribute("@keydown.enter.prevent", "open = false; focusButton()");
									props.set_attribute("@keyup.space.prevent", "open = false; focusButton()");
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
										props.set_attribute("x-state:on", "Active");
										props.set_attribute("x-state:off", "Not Active");
										props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 0 }");
										props.set_attribute("role", "menuitem");
										props.set_attribute("tabindex", "-1");
										props.set_attribute("id", "user-menu-item-0");
										props.set_attribute("@mouseenter", "onMouseEnter($event)");
										props.set_attribute("@mousemove", "onMouseMove($event, 0)");
										props.set_attribute("@mouseleave", "onMouseLeave($event)");
										props.set_attribute("@click", "open = false; focusButton()");
										
										props.child("0", Label).run(|props| props.text("Your Profile"));
									});
									props.child("3", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
										props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 1 }");
										props.set_attribute("role", "menuitem");
										props.set_attribute("tabindex", "-1");
										props.set_attribute("id", "user-menu-item-1");
										props.set_attribute("@mouseenter", "onMouseEnter($event)");
										props.set_attribute("@mousemove", "onMouseMove($event, 1)");
										props.set_attribute("@mouseleave", "onMouseLeave($event)");
										props.set_attribute("@click", "open = false; focusButton()");
										
										props.child("0", Label).run(|props| props.text("Settings"));
									});
									props.child("5", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
										props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 2 }");
										props.set_attribute("role", "menuitem");
										props.set_attribute("tabindex", "-1");
										props.set_attribute("id", "user-menu-item-2");
										props.set_attribute("@mouseenter", "onMouseEnter($event)");
										props.set_attribute("@mousemove", "onMouseMove($event, 2)");
										props.set_attribute("@mouseleave", "onMouseLeave($event)");
										props.set_attribute("@click", "open = false; focusButton()");
										
										props.child("0", Label).run(|props| props.text("Sign out"));
									});
								});
							});
						});
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.set_attribute("x-description", "Mobile menu, show/hide based on menu state.");
					props.styles(&[Style::OnScreen(Screen::Small, &[Style::Noop("hidden")])]);
					props.set_attribute("id", "mobile-menu");
					props.set_attribute("x-show", "open");
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::SpaceY(Size::Exact(4)), Style::PaddingX(Size::Exact(8)), Style::PaddingBottom(Size::Exact(12)), Style::PaddingTop(Size::Exact(8))]);
						
						props.child("1", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Color(Color::Fg(100)), Style::Noop("text-white"), Style::Block, Style::Noop("rounded-md"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-base"), Style::Noop("font-medium")]);
							props.set_attribute("x-state:on", "Current");
							props.set_attribute("x-state:off", "Default");
							props.set_attribute("aria-current", "page");
							props.set_attribute("x-state-description", r#"Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
							
							props.child("0", Label).run(|props| props.text("Dashboard"));
						});
						props.child("3", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::TextColor(Color::Fg(33)), Style::OnHover(&[Style::Color(Color::Fg(78))]), Style::OnHover(&[Style::Noop("text-white")]), Style::Block, Style::Noop("rounded-md"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-base"), Style::Noop("font-medium")]);
							props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
							
							props.child("0", Label).run(|props| props.text("Team"));
						});
						props.child("5", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::TextColor(Color::Fg(33)), Style::OnHover(&[Style::Color(Color::Fg(78))]), Style::OnHover(&[Style::Noop("text-white")]), Style::Block, Style::Noop("rounded-md"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-base"), Style::Noop("font-medium")]);
							props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
							
							props.child("0", Label).run(|props| props.text("Projects"));
						});
						props.child("7", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::TextColor(Color::Fg(33)), Style::OnHover(&[Style::Color(Color::Fg(78))]), Style::OnHover(&[Style::Noop("text-white")]), Style::Block, Style::Noop("rounded-md"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-base"), Style::Noop("font-medium")]);
							props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
							
							props.child("0", Label).run(|props| props.text("Calendar"));
						});
					});
				});
			});
		});
	}
}
